use std::sync::Arc;

use crate::{
    api::{KeycloakAuthBuilder, KeycloakClient},
    crd::KeycloakInstance,
    error::{Error, Result},
    util::SecretUtils,
};
use async_trait::async_trait;
use k8s_openapi::api::core::v1::Secret;
use kube::{
    runtime::{controller::Action, Controller},
    Api, ResourceExt,
};
use log::{debug, info};
use reqwest::{Method, Response, StatusCode};
use serde_json::Value;

use super::controller_runner::LifetimeController;
use crate::crd::KeycloakApiObject;

#[derive(Debug, Default)]
pub struct KeycloakApiController {}

impl KeycloakApiController {
    async fn keycloak_client(
        client: kube::Client,
        resource: &KeycloakApiObject,
    ) -> Result<KeycloakClient> {
        let ns = resource.namespace().ok_or(Error::NoNamespace)?;
        let secret_api = Api::<Secret>::namespaced(client.clone(), &ns);
        let instance_api =
            Api::<KeycloakInstance>::namespaced(client.clone(), &ns);

        let instance_name = &resource.spec.api.keycloak_selector.name;
        let instance = instance_api.get(instance_name).await?;
        // TODO: the secret name can be customized
        let token = secret_api
            .get(&instance.token_secret_name())
            .await?
            .token(&instance)?;

        Ok(KeycloakAuthBuilder::default()
            .url(&instance.spec.base_url)
            //.client_id(&instance.spec.client_id)
            //.client_secret(&instance.spec.client_secret)
            .build()?
            .into_client(token))
    }

    async fn request(
        &self,
        client: &KeycloakClient,
        method: Method,
        path: &str,
        payload: &Value,
    ) -> Result<Response> {
        let request = client.request(method, path);
        let request = if payload == &Value::Null {
            request
        } else {
            request.json(payload)
        };
        debug!("Request: {:?}", request);
        Ok(request.send().await?.error_for_status()?)
    }
}

#[async_trait]
impl LifetimeController for KeycloakApiController {
    type Resource = KeycloakApiObject;

    fn prepare(
        &self,
        controller: Controller<Self::Resource>,
        _client: &kube::Client,
    ) -> Controller<Self::Resource> {
        controller
    }

    async fn apply(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let path = &resource.spec.path;
        let keycloak = Self::keycloak_client(client.clone(), &resource).await?;
        let payload = resource.resolve(client).await?;
        // First try to PUT, if we get a 404, try to POST
        let response =
            match self.request(&keycloak, Method::PUT, path, &payload).await {
                Err(Error::ReqwestError(e)) => {
                    if e.status() == Some(StatusCode::NOT_FOUND) {
                        let path = path.rsplit_once('/').unwrap().0;
                        self.request(&keycloak, Method::POST, path, &payload)
                            .await
                    } else {
                        Err(e)?
                    }
                }
                r => r,
            }?;
        info!("Response: {:?}", response.text().await?);
        // TODO: handle errors
        Ok(Action::await_change())
    }

    async fn cleanup(
        &self,
        client: &kube::Client,
        resource: Arc<Self::Resource>,
    ) -> Result<Action> {
        let path = &resource.spec.path;
        let keycloak = Self::keycloak_client(client.clone(), &resource).await?;
        // TODO: handle errors
        let _response = self
            .request(&keycloak, Method::DELETE, path, &Value::Null)
            .await?;

        Ok(Action::await_change())
    }
}
