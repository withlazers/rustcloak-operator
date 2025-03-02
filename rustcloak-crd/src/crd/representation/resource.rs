use crate::keycloak_types::ResourceRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    crd::namespace_scope, impl_object, schema_patch, traits::impl_instance_ref,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::ClientRef;

namespace_scope! {
    "KeycloakResource", "kcrs" {
        #[kube(
            doc = "resource to define a Resource within a [KeyclaokClient](./keycloakclient.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
        )]
        /// the KeycloakResource resource
        pub struct KeycloakResourceSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: ClientRef,
            #[schemars(schema_with = "schema")]
            pub definition: ResourceRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

impl_object!("resource" <ClientRef> / |_d| {"authz/resource-server/resource"} / (id => "_id") for KeycloakResourceSpec => ResourceRepresentation);

impl_instance_ref!(KeycloakResource);

schema_patch!(KeycloakResourceSpec: |s| {
    s.prop("scopes")
        .array_item()
        .prop("policies")
        .array_item()
        .remove("scopesData")
        .remove("resourcesData")
        .additional_properties();
    s.prop("scopes").array_item().remove("resources");
    s.prop("scopesUma")
        .array_item()
        .prop("policies")
        .array_item()
        .remove("resourcesData")
        .remove("scopesData")
        .additional_properties();
    s.prop("scopesUma")
        .array_item()
        .remove("resources")
        .additional_properties();
});
