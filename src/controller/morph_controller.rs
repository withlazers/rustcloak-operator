use std::sync::Arc;

use super::controller_runner::LifecycleController;
use crate::{
    app_id,
    crd::{
        ChildOf, HasApiObject, HasParentType, KeycloakApiEndpoint,
        KeycloakApiObject, KeycloakApiObjectSpec,
    },
    endpoint::{HasHierarchy, Hierarchy, HierarchyQuery},
    error::{Error, Result},
    morph::Morph,
};
use async_trait::async_trait;
use k8s_openapi::NamespaceResourceScope;
use kube::{
    api::{ObjectMeta, Patch, PatchParams},
    runtime::{controller::Action, watcher, Controller},
    Api, Resource, ResourceExt,
};
use log::debug;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

#[derive(Debug)]
pub struct MorphController<T: HasParentType> {
    phantom: std::marker::PhantomData<T>,
}

impl<T: HasParentType> Default for MorphController<T> {
    fn default() -> Self {
        Self {
            phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<R> LifecycleController for MorphController<R>
where
    R: HasParentType
        + Morph
        + HasHierarchy
        + HasApiObject
        + HasParentType
        + ChildOf
        + Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + Send
        + Sync
        + 'static
        + Clone
        + std::fmt::Debug
        + Serialize
        + DeserializeOwned,
    R::Parent: Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + Clone
        + std::fmt::Debug
        + DeserializeOwned
        + Send
        + Sync
        + 'static,
    R::HierarchyParent: HierarchyQuery<Object = R::Parent> + Sync + Send,
    R::ParentRefType: AsRef<str> + Sync + Send,
{
    type Resource = R;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        client: &kube::Client,
    ) -> Controller<Self::Resource> {
        controller.owns(
            Api::<KeycloakApiObject>::all(client.clone()),
            watcher::Config::default(),
        )
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let admin_api: Api<KeycloakApiObject> =
            Api::namespaced(client.clone(), &ns);
        let name = format!("{}-{}", R::prefix(), resource.name_unchecked());
        let owner_ref = resource.owner_ref(&()).unwrap();

        let primary_key = R::primary_key();
        let mut payload = resource.payload()?;
        let vars = resource.variables()?;
        payload
            .as_object_mut()
            .as_mut()
            .unwrap()
            .remove(primary_key);
        let primary_key_value = resource.primary_key_value();
        let immutable_payload = json!({
            primary_key: primary_key_value,
        })
        .into();
        let payload = payload.into();

        let hierarchy =
            Hierarchy::<Self::Resource>::query(&resource, client.clone())
                .await?;
        let endpoint = KeycloakApiEndpoint {
            instance_ref: hierarchy.instance_ref().to_string().into(),
            path: hierarchy.path().into(),
        };
        //let endpoint = resource.resolve(client.clone()).await?;
        debug!("Resolved endpoint: {:?}", endpoint);

        let api_object = KeycloakApiObject {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: Some(ns.clone()),
                owner_references: Some(vec![owner_ref]),
                ..Default::default()
            },
            spec: KeycloakApiObjectSpec {
                endpoint,
                options: resource.options().cloned(),
                immutable_payload,
                payload,
                vars,
            },
            status: Default::default(),
        };

        admin_api
            .patch(
                &name,
                &PatchParams::apply(app_id!()),
                &Patch::Apply(api_object),
            )
            .await?;

        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        _client: &kube::Client,
        _resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        Ok(Action::await_change())
    }
}
