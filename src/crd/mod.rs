mod api_object;
mod container_types;
mod instance;
mod realm;

pub use api_object::*;
pub use container_types::*;
pub use instance::*;
pub use realm::*;

use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, ObjectValidation, Schema, SchemaObject},
    JsonSchema,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct KeycloakApiStatus {
    ready: bool,
    status: String,
    #[serde(default)]
    code: u32,
    #[serde(default)]
    message: String,
}

impl From<&Error> for KeycloakApiStatus {
    fn from(err: &Error) -> Self {
        KeycloakApiStatus {
            ready: false,
            status: "error".to_string(),
            code: 0,
            message: err.to_string(),
        }
    }
}

impl KeycloakApiStatus {
    pub fn from_error(e: Error) -> Value {
        [(
            "status".to_string(),
            serde_json::to_value(KeycloakApiStatus::from(&e)).unwrap(),
        )]
        .into_iter()
        .collect::<serde_json::Map<_, _>>()
        .into()
    }
}

pub trait WithStatus<T> {
    fn status(&self) -> Option<&T>;
}

fn schema_extra(_generator: &mut SchemaGenerator) -> Schema {
    //println!("{:#?}", _generator.root_schema_for::<Option<std::collections::HashMap<String, serde_json::Value>>>());
    //println!("{:#?}", _generator.root_schema_for::<std::collections::HashMap<String, serde_json::Value>>());
    Schema::Object(SchemaObject {
        instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(
            InstanceType::Object,
        ))),
        object: Some(Box::new(ObjectValidation {
            additional_properties: Some(Box::new(Schema::Bool(true))),
            ..Default::default()
        })),
        extensions: [("nullable".to_string(), Value::Bool(true))].into(),
        ..Default::default()
    })
}
