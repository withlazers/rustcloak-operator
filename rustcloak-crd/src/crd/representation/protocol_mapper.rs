use crate::keycloak_types::ProtocolMapperRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    crd::namespace_scope, impl_object, schema_patch, traits::impl_instance_ref,
};
use either::Either;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{ClientRef, ClientScopeRef};

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

type ParentRef = Either<ClientRef, ClientScopeRef>;

impl_object!("pm" <ParentRef> / |_d| {"protocol-mappers/models"} / id for KeycloakProtocolMapperSpec => ProtocolMapperRepresentation);

impl_instance_ref!(KeycloakProtocolMapper);

schema_patch!(KeycloakProtocolMapperSpec);
