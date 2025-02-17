use super::controller_runner::LifecycleController;
use crate::crd::KeycloakApiEndpointPath;
use crate::crd::KeycloakApiObject;
use crate::util::RefWatcher;
use crate::{
    api::KeycloakApiClient,
    app_id,
    crd::{KeycloakApiStatus, KeycloakInstance},
    error::{Error, Result},
    util::K8sKeycloakBuilder,
};
use async_stream::stream;
use async_trait::async_trait;
use k8s_openapi::{
    api::core::v1::{ConfigMap, Secret},
    DeepMerge,
};
use kube::runtime::watcher;
use kube::Resource;
use kube::{
    api::PatchParams,
    runtime::{controller::Action, Controller},
    Api, ResourceExt,
};
use log::warn;
use reqwest::StatusCode;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Notify;

/// This controller is responsible for applying the desired keycloak state in kubernetes to a
/// keycloak instance.
#[derive(Debug, Default)]
pub struct KeycloakApiObjectController {
    reconcile_notify: Arc<Notify>,
    secret_refs: Arc<RefWatcher<KeycloakApiObject, Secret>>,
    config_map_refs: Arc<RefWatcher<KeycloakApiObject, ConfigMap>>,
}

impl KeycloakApiObjectController {
    async fn resolve_path(
        &self,
        client: &kube::Client,
        ns: &str,
        resource: &KeycloakApiObject,
        stack: Vec<String>,
    ) -> Result<String> {
        let (name, sub_path) = match &resource.spec.endpoint.path_def {
            KeycloakApiEndpointPath::Path(path) => return Ok(path.to_string()),
            KeycloakApiEndpointPath::Parent {
                parent_ref,
                sub_path,
            } => (parent_ref, sub_path),
        };

        let api = Api::<KeycloakApiObject>::namespaced(client.clone(), ns);
        let parent = api
            .get_opt(name)
            .await?
            .ok_or(Error::NoParent(ns.to_string(), name.to_string()))?;

        let is_recursive = stack.contains(&parent.name_any());

        let mut stack = stack;
        stack.push(parent.name_any());

        if is_recursive {
            return Err(Error::RecursiveParent(
                ns.to_string(),
                stack.join(" -> "),
            ));
        }

        let path =
            Box::pin(self.resolve_path(client, ns, &parent, stack)).await?;
        Ok(format!(
            "{}/{}",
            path.trim_end_matches('/'),
            sub_path.trim_start_matches('/')
        ))
    }

    async fn keycloak(
        client: &kube::Client,
        resource: &KeycloakApiObject,
    ) -> Result<KeycloakApiClient> {
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let instance_api =
            Api::<KeycloakInstance>::namespaced(client.clone(), &ns);

        let instance_ref = &resource.spec.endpoint.instance_ref;
        let instance = instance_api
            .get_opt(instance_ref)
            .await?
            .ok_or(Error::NoInstance(ns, instance_ref.to_string()))?;

        K8sKeycloakBuilder::new(&instance, client)
            .with_token()
            .await
    }
}

#[async_trait]
impl LifecycleController for KeycloakApiObjectController {
    type Resource = KeycloakApiObject;
    const MODULE_PATH: &'static str = module_path!();

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource> {
        let notify = self.reconcile_notify.clone();
        let secret_refs = self.secret_refs.clone();
        let config_map_refs = self.config_map_refs.clone();
        let secret_api = Api::<Secret>::all(client.clone());
        let config_map_api = Api::<ConfigMap>::all(client.clone());
        controller
            .reconcile_all_on(stream! {
                loop {
                    notify.notified().await;
                    yield;
                }
            })
            .watches(secret_api, watcher::Config::default(), move |secret| {
                secret_refs.watch(&secret)
            })
            .watches(
                config_map_api,
                watcher::Config::default(),
                move |config_map| config_map_refs.watch(&config_map),
            )
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let name = resource.name_unchecked();
        let api = Api::<KeycloakApiObject>::namespaced(client.clone(), &ns);
        let keycloak = Self::keycloak(client, &resource).await?;
        let mut payload = resource.resolve(client).await?;
        let immutable_payload: Value =
            serde_yaml::from_str(&resource.spec.immutable_payload.0)?;
        payload.merge_from(immutable_payload.clone());
        let mut success = false;
        let kind = KeycloakApiObject::kind(&());

        if let Some(path) = resource
            .status
            .as_ref()
            .and_then(|x| x.resource_path.as_ref())
        {
            match keycloak.put(path, &payload).await {
                Ok(_) => {
                    success = true;
                    api.patch_status(
                        &name,
                        &PatchParams::apply(app_id!()),
                        &KeycloakApiStatus::ok("Applied").into(),
                    )
                    .await?;
                }
                Err(Error::KeycloakError(StatusCode::NOT_FOUND, m)) => {
                    warn!(
                        kind = kind,
                        name = name,
                        namespace = ns,
                        path = path;
                        "Failed to update resource at path, try recreating. (Message: {})",
                        m
                    );
                }
                x => x?,
            }
        }

        if !success {
            let path = self
                .resolve_path(client, &ns, &resource, vec![resource.name_any()])
                .await?;
            let resource_path = keycloak.post_location(&path, &payload).await?;
            let mut status = resource.status.clone().unwrap_or_default();
            status.resource_path = Some(resource_path);

            api.patch_status(
                &name,
                &PatchParams::apply(app_id!()),
                &status.into(),
            )
            .await?;
        }

        let ref_iter = resource
            .spec
            .vars
            .iter()
            .filter_map(|v| v.value_from.as_ref());
        let secret_refs = ref_iter
            .clone()
            .filter_map(|v| v.secret_key_ref.as_ref().map(|r| &r.name));
        self.secret_refs.add(&resource, secret_refs);
        let config_map_refs = ref_iter
            .filter_map(|v| v.config_map_key_ref.as_ref().map(|r| &r.name));
        self.config_map_refs.add(&resource, config_map_refs);

        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let kind = KeycloakApiObject::kind(&());
        let name = resource.name_unchecked();
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let Some(path) = resource
            .status
            .as_ref()
            .and_then(|x| x.clone().resource_path)
        else {
            // If the resource has no resource URL we expect that it never got created, so it's
            // safe to delete the resource.
            return Ok(Action::await_change());
        };
        let keycloak = match Self::keycloak(client, &resource).await {
            Ok(k) => k,
            Err(Error::NoInstance(_, _)) => {
                warn!(
                    kind = kind,
                    name = name,
                    namespace = ns,
                    path = path;
                    "Keycloak instance not found, assuming you want to unmanage the whole keycloak instance."
                );
                return Ok(Action::await_change());
            }
            Err(e) => Err(e)?,
        };
        match keycloak.delete(&path).await {
            Err(Error::KeycloakError(StatusCode::NOT_FOUND, m)) => {
                warn!(
                    kind = kind,
                    name = name,
                    namespace = ns,
                    path = path;
                    "Resource not found, assuming it's already deleted. Message: {}", m);
            }
            x => x?,
        }

        self.secret_refs.remove(&resource);
        self.config_map_refs.remove(&resource);

        // If we delete a resource, we let the controller reconcile all objects, so that
        // the resources that depend on the deleted resource are starting to fail.
        self.reconcile_notify.notify_one();

        Ok(Action::await_change())
    }
}
