use std::collections::BTreeMap;

use k8s_openapi::api::core::v1::EnvVarSource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakApiPatchList {
    /// Defines additional values that can be loaded from secrets or configmaps. Field selectors
    /// are not supported. For more informations see [the patches documentation](../configuration/patches.md).
    ///
    pub patch_from: BTreeMap<String, KeycloakApiPatch>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum KeycloakApiPatchValueAs {
    #[default]
    Auto,
    String,
    Number,
    Yaml,
    Json,
    Bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakApiPatch {
    #[serde(flatten)]
    pub source: EnvVarSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_as: Option<KeycloakApiPatchValueAs>,
}
