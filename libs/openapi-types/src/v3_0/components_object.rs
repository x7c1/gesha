use crate::v3_0::ReferenceObject;
use std::collections::HashMap;

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#componentsObject
#[derive(Debug)]
pub struct ComponentsObject {
    pub schemas: SchemasObject,
}

#[derive(Debug)]
pub struct SchemasObject(HashMap<SchemaFieldName, SchemaCase>);

impl SchemasObject {
    pub fn new(map: HashMap<SchemaFieldName, SchemaCase>) -> Self {
        SchemasObject(map)
    }
}

/// > All the fixed fields declared above are objects
/// > that MUST use keys that match the regular expression: ^[a-zA-Z0-9\.\-_]+$.
#[derive(Debug)]
pub struct SchemaFieldName(String);

/// Schema Object | Reference Object
#[derive(Debug)]
pub enum SchemaCase {
    Schema(SchemaObject),
    Reference(ReferenceObject),
}

#[derive(Debug)]
pub struct SchemaObject {}
