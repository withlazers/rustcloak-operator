use crate::keycloak_types::ComponentRepresentation;
use crate::refs::ref_type;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    crd::namespace_scope, impl_object, schema_patch, traits::impl_endpoint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::RealmRef;

namespace_scope! {
    "KeycloakComponent", "kcco" {
        #[kube(
            doc = "resource to define a Component within a [KeycloakRealm](./keycloakrealm.md)",
            group = "rustcloak.k8s.eboland.de",
            version = "v1beta1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
        )]
        /// the KeycloakComponent resource
        pub struct KeycloakComponentSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            #[serde(flatten)]
            pub parent_ref: RealmRef,
            #[schemars(schema_with = "schema")]
            pub definition: ComponentRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

impl_object!("component" <RealmRef> / |_d| {"components"} / id for KeycloakComponentSpec => ComponentRepresentation);

impl_endpoint!(KeycloakComponent);

schema_patch!(KeycloakComponentSpec);

ref_type!(ComponentRef, component_ref, KeycloakComponent);
