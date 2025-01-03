use crate::crd::{
    api_object_impl, route_impl, schema_patch, KeycloakApiObjectOptions,
    KeycloakApiPatchList, KeycloakApiStatus,
};
use keycloak::types::AuthenticationFlowRepresentation;
use kube::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakAuthenticationFlow",
    shortname = "kcaf",
    doc = "resource to define an Authentication Flow within a [KeycloakRealm](./keycloakrealm.md)",
    group = "rustcloak.k8s.eboland.de",
    version = "v1",
    status = "KeycloakApiStatus",
    category = "keycloak",
    category = "all",
    namespaced,
    printcolumn = r#"{
            "name":"Ready",
            "type":"boolean",
            "description":"true if the realm is ready",
            "jsonPath":".status.ready"
        }"#,
    printcolumn = r#"{
            "name":"Status",
            "type":"string",
            "description":"Status String of the resource",
            "jsonPath":".status.status"
        }"#,
    printcolumn = r#"{
            "name":"Age",
            "type":"date",
            "description":"time since the realm was created",
            "jsonPath":".metadata.creationTimestamp"
        }"#
)]
#[serde(rename_all = "camelCase")]
/// the KeycloakAuthenticationFlow resource
pub struct KeycloakAuthenticationFlowSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    /// the name of the kubernetes object that created the realm.
    pub realm_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: AuthenticationFlowRepresentation,
    #[serde(default, flatten)]
    pub patches: Option<KeycloakApiPatchList>,
}

api_object_impl!(
    KeycloakAuthenticationFlow,
    AuthenticationFlowRepresentation,
    "realm"
);

route_impl!(KeycloakRealm / "authentication/flows" / id: KeycloakAuthenticationFlow .. realm_ref: String);

schema_patch!(KeycloakAuthenticationFlow);
