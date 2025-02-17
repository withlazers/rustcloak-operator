use crate::crd::{
    api_object_impl, schema_patch, KeycloakApiObjectOptions,
    KeycloakApiPatchList, KeycloakApiStatus, KeycloakRealm,
};
use keycloak::types::IdentityProviderRepresentation;
use kube::CustomResource;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    kind = "KeycloakIdentityProvider",
    shortname = "kcip",
    doc = "resource to define a identity provider in a [KeyclaokRealm](./keycloakrealm.md)",
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
/// the KeycloakIdentityProvider resource
pub struct KeycloakIdentityProviderSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<KeycloakApiObjectOptions>,
    /// the name of the kubernetes object that created the realm.
    pub realm_ref: String,
    #[schemars(schema_with = "schema")]
    pub definition: IdentityProviderRepresentation,
    #[serde(default, flatten)]
    pub patches: Option<KeycloakApiPatchList>,
}

api_object_impl!(
    KeycloakIdentityProvider,
    IdentityProviderRepresentation,
    "idp"
);

crate::crd::route_impl!(KeycloakRealm / "identity-provider/instances" / alias: KeycloakIdentityProvider .. realm_ref: String);

schema_patch!(KeycloakIdentityProvider);
