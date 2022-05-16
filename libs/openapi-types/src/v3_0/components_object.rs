use crate::v3_0::ReferenceObject;
use std::collections::HashMap;

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#componentsObject
#[derive(Debug)]
pub struct ComponentsObject {
    pub schemas: Option<SchemasObject>,
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
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct SchemaFieldName(String);

impl SchemaFieldName {
    pub fn new<A: Into<String>>(a: A) -> Self {
        SchemaFieldName(a.into())
    }
}

/// Schema Object | Reference Object
#[derive(Debug)]
pub enum SchemaCase {
    Schema(SchemaObject),
    Reference(ReferenceObject),
}

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#schemaObject
#[derive(Debug)]
pub struct SchemaObject {
    /// 'type'
    pub type_name: Option<String>,
    pub properties: Option<Vec<(SchemaFieldName, SchemaCase)>>,
    pub required: Vec<String>,
}
