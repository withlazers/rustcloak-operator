use crate::keycloak_types::ClientScopeRepresentation;
use crate::{
    ImmutableString, KeycloakApiObjectOptions, KeycloakApiPatchList,
    KeycloakApiStatus, KeycloakRealm, impl_object, macros::namespace_scope,
    schema_patch, traits::impl_instance_ref,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakClientScope", "kccs" {
        #[kube(
            doc = "resource to define a Scope within a [KeycloakClient](./keycloakclient.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
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
        /// the KeycloakClientScope resource
        pub struct KeycloakClientScopeSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            /// the name of the kubernetes object that created the realm.
            pub realm_ref: ImmutableString,
            // TODO: is_template should be immutable. We can't do immutable options yet.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub is_template: Option<bool>,
            #[schemars(schema_with = "schema")]
            pub definition: ClientScopeRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

impl_instance_ref!(KeycloakClientScope);

impl_object!("scopespec" <realm_ref: String => KeycloakRealm> / |d| {
    if d.is_template == Some(true) {
        "client-scopes"
    } else {
        "client-templates"
    }
} / id for KeycloakClientScopeSpec => ClientScopeRepresentation);

schema_patch!(KeycloakClientScopeSpec);
