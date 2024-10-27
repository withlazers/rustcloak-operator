use crate::crd::{
    api_object_impl, schema_patch, KeycloakApiObjectOptions, KeycloakApiStatus,
};
use keycloak::types::AuthenticatorConfigRepresentation;
use kube_derive::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakAuthenticatorConfig",
    shortname = "kcac",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakAuthenticatorConfigSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    pub realm_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: AuthenticatorConfigRepresentation,
}

api_object_impl!(
    KeycloakAuthenticatorConfig,
    AuthenticatorConfigRepresentation,
    "authconfig"
);

crate::crd::route_impl!(KeycloakRealm / "authentication/config" / id: KeycloakAuthenticatorConfig .. realm_ref: String);

schema_patch!(KeycloakAuthenticatorConfig);
