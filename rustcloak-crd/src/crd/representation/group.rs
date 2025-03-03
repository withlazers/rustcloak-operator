use crate::keycloak_types::GroupRepresentation;
use crate::{
    KeycloakApiObjectOptions, KeycloakApiPatchList, KeycloakApiStatus,
    impl_object,
    macros::namespace_scope,
    refs::{RealmRef, SubGroupRef},
    schema_patch,
    traits::impl_instance_ref,
};
use either::Either;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::KeycloakRealm;

namespace_scope! {
    "KeycloakGroup", "kcg" {
        #[kube(
            doc = "resource to define a Group within a [KeycloakRealm](./keycloakrealm.md)",
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
        /// the KeycloakGroup resource
        pub struct KeycloakGroupSpec {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub options: Option<KeycloakApiObjectOptions>,
            /// the name of the kubernetes object that created the realm.
            #[serde(flatten)]
            pub parent_ref: ParentRef,
            #[schemars(schema_with = "schema")]
            pub definition: GroupRepresentation,
            #[serde(default, flatten)]
            pub patches: Option<KeycloakApiPatchList>,
        }
    }
}

type ParentRef = Either<RealmRef, SubGroupRef>;
type Parent = Either<KeycloakRealm, KeycloakGroup>;

impl_object!("group" <parent_ref: ParentRef => Parent> / |d| {
    if d.parent_ref.is_left() {
        "groups"
    } else {
        "children"
    }
} / id for KeycloakGroupSpec => GroupRepresentation);

impl_instance_ref!(KeycloakGroup);

schema_patch!(KeycloakGroupSpec: |s| {
    s.remove("subGroups");
});
