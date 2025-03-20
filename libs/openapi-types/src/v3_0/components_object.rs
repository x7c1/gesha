use crate::v3_0::ComponentName;
use crate::v3_0::RequestBodiesObject;
use crate::v3_0::schema_case::SchemaCase;
use indexmap::IndexMap;

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#componentsObject
#[derive(Debug)]
pub struct ComponentsObject {
    pub request_bodies: Option<RequestBodiesObject>,
    pub schemas: Option<SchemasObject>,
}

pub type SchemasObject = IndexMap<ComponentName, SchemaCase>;
