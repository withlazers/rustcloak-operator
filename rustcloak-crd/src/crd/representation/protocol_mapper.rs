use crate::keycloak_types::ProtocolMapperRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    KeycloakClient, KeycloakClientScope, impl_object,
    macros::namespace_scope,
    refs::{ClientRef, ClientScopeRef},
    schema_patch,
    traits::impl_instance_ref,
};
use either::Either;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

namespace_scope! {
    "KeycloakProtocolMapper",
    "kcpm" {
        #[kube(
            doc = "resource to define a Protocol Mapper within either a [KeycloakClient](./keycloakclient.md) or a [KeycloakClientScope](./keycloakclientscope.md)",
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
        /// the KeycloakProtocolMapper resource
        pub struct KeycloakProtocolMapperSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            /// the name of the kubernetes object that created the parent resource.
            pub parent_ref: ParentRef,
            #[schemars(schema_with = "schema")]
            pub definition: ProtocolMapperRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

type Parents = Either<KeycloakClient, KeycloakClientScope>;
type ParentRef = Either<ClientRef, ClientScopeRef>;

impl_object!("pm" <parent_ref: ParentRef => Parents> / |_d| {"protocol-mappers/models"} / id for KeycloakProtocolMapperSpec => ProtocolMapperRepresentation);

impl_instance_ref!(KeycloakProtocolMapper);

schema_patch!(KeycloakProtocolMapperSpec);
