use crate::crd::{HasApiObject, KeycloakInstance};
use crate::error::{Error, Result};
use async_trait::async_trait;
use k8s_openapi::NamespaceResourceScope;
use kube::{Api, Resource};
use serde::de::DeserializeOwned;

pub struct Query<T>(T);

#[async_trait]
impl<O> up_impl::Query for Query<O>
where
    O: Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + HasApiObject
        + Clone
        + std::fmt::Debug
        + DeserializeOwned
        + Send
        + Sync,
{
    type UserData = (kube::Client, String);
    type Error = Error;
    type Key = String;
    type Output = O;

    async fn query(
        key: Self::Key,
        user_data: &Self::UserData,
    ) -> Result<Self::Output> {
        let (client, ns) = user_data;
        let api = Api::<O>::namespaced(client.clone(), &ns);
        Ok(api.get(&key).await?)
    }
}

#[async_trait]
impl up_impl::Query for Query<KeycloakInstance> {
    type UserData = (kube::Client, String);
    type Error = Error;
    type Key = String;
    type Output = KeycloakInstance;

    async fn query(
        key: Self::Key,
        user_data: &Self::UserData,
    ) -> Result<Self::Output> {
        let (client, ns) = user_data;
        let api = Api::<KeycloakInstance>::namespaced(client.clone(), &ns);
        Ok(api.get(&key).await?)
    }
}
