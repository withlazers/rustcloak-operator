use crate::marker::{HasMarker, ResourceMarker};
use crate::refs::ref_type;
use crate::traits::{Endpoint, SecretKeyNames};

use super::KeycloakApiStatusEndpoint;
use super::{KeycloakApiStatus, both_scopes};
use kube::{CustomResource, Resource};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakInstanceCredentialReference {
    pub create: Option<bool>,
    pub secret_name: String,
    pub username_key: Option<String>,
    pub password_key: Option<String>,
}

impl SecretKeyNames<2> for KeycloakInstanceCredentialReference {
    const DEFAULTS: [&'static str; 2] = ["user", "password"];

    fn secret_key_names_opts(&self) -> Option<[&Option<String>; 2]> {
        Some([&self.username_key, &self.password_key])
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakInstanceTokenReference {
    pub secret_name: Option<String>,
    pub token_key: Option<String>,
    pub expires_key: Option<String>,
}

impl SecretKeyNames<2> for Option<KeycloakInstanceTokenReference> {
    const DEFAULTS: [&'static str; 2] = ["token", "expires"];

    fn secret_key_names_opts(&self) -> Option<[&Option<String>; 2]> {
        self.as_ref().map(|x| [&x.token_key, &x.expires_key])
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakInstanceClient {
    pub id: String,
    pub secret: Option<String>,
}

both_scopes! {
    "KeycloakInstance", "kci", "ClusterKeycloakInstance", "ckci", ClusterKeycloakInstanceSpec {
        #[kube(
            doc = "This resource makes a Keycloak instance known to the operator",
            group = "rustcloak.k8s.eboland.de",
            version = "v1",
            status = "KeycloakApiStatus",
            category = "keycloak",
            category = "all",
            printcolumn = r#"{
                    "name":"Base URL",
                    "type":"string",
                    "description":"The base URL of the Keycloak instance",
                    "jsonPath":".spec.baseUrl"
                }"#,
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
                }"#
        )]
        pub struct KeycloakInstanceSpec {
            pub base_url: String,
            pub realm: Option<String>,
            pub credentials: KeycloakInstanceCredentialReference,
            pub token: Option<KeycloakInstanceTokenReference>,
            pub client: Option<KeycloakInstanceClient>,
        }
    }
}

impl KeycloakInstanceSpec {
    pub fn token_secret_ref(&self) -> Option<&KeycloakInstanceTokenReference> {
        self.token.as_ref()
    }

    pub fn token_secret_name(&self, name: String) -> String {
        if let Some(name) =
            self.token.as_ref().and_then(|x| x.secret_name.as_ref())
        {
            name.to_string()
        } else {
            format!("{}-api-token", name)
        }
    }

    pub fn credential_secret_name(&self) -> &str {
        self.credentials.secret_name.as_str()
    }
}

impl Endpoint for KeycloakInstance {
    fn endpoint(&self) -> Option<&KeycloakApiStatusEndpoint> {
        None
    }
}

impl HasMarker for KeycloakInstance {
    type Marker = ResourceMarker<<Self as Resource>::Scope>;
}

impl Endpoint for ClusterKeycloakInstance {
    fn endpoint(&self) -> Option<&KeycloakApiStatusEndpoint> {
        None
    }
}

impl HasMarker for ClusterKeycloakInstance {
    type Marker = ResourceMarker<<Self as Resource>::Scope>;
}

ref_type!(InstanceRef, instance_ref, KeycloakInstance);
//ref_type!(NamespacedInstanceRef, instance_ref, KeycloakInstance);
ref_type!(
    ClusterInstanceRef,
    cluster_instance_ref,
    ClusterKeycloakInstance
);
//pub type InstanceRef = Either<NamespacedInstanceRef, ClusterInstanceRef>;
