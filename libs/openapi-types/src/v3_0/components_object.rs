use crate::v3_0::ReferenceObject;
use indexmap::{IndexMap, IndexSet};
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
    /// > type - Value MUST be a string.
    /// > Multiple types via an array are not supported.
    pub type_name: Option<String>,

    pub properties: Option<SchemaProperties>,

    pub required: Option<RequiredSchemaFields>,
}

/// > properties - Property definitions MUST be a Schema Object
/// > and not a standard JSON Schema (inline or referenced).
///
/// see also: https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.16
pub type SchemaProperties = IndexMap<SchemaFieldName, SchemaCase>;

/// https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.15
/// >The value of this keyword MUST be an array.  This array MUST have at
/// >least one element. Elements of this array MUST be strings, and MUST
/// >be unique.
#[derive(Debug)]
pub struct RequiredSchemaFields(IndexSet<String>);

impl RequiredSchemaFields {
    pub fn new(fields: IndexSet<String>) -> Self {
        Self(fields)
    }
}
