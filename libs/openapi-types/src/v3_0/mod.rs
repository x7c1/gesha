mod operation_object;

pub use operation_object::{
    HttpStatusCode, OperationObject, ResponseCase, ResponseObject, ResponsesObject,
};
use std::fmt::{Display, Formatter};

mod paths_object;
pub use paths_object::{PathFieldName, PathItemObject, PathsObject};

mod components_object;
pub use components_object::{
    ComponentsObject, RequiredSchemaFields, SchemaCase, SchemaFieldName, SchemaObject,
    SchemaProperties, SchemasObject,
};

/// OpenAPI Document
/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#schema
#[derive(Debug)]
pub struct Document {
    pub openapi: String,
    pub info: InfoObject,
    pub paths: PathsObject,
    pub components: Option<ComponentsObject>,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#infoObject
#[derive(Debug)]
pub struct InfoObject {
    pub title: String,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#referenceObject
#[derive(Debug)]
pub struct ReferenceObject(String);

impl ReferenceObject {
    pub fn new<A: Into<String>>(a: A) -> Self {
        ReferenceObject(a.into())
    }
}

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#dataTypes
#[derive(Debug)]
pub enum OpenApiDataType {
    Object,
    String,
    Integer,
    Number,
    Array,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#dataTypes
#[derive(Debug)]
pub enum FormatModifier {
    Int32,
    Int64,
    Float,
    // TODO:
    // > the format property is an open string-valued property,
    // > and can have any value. Formats such as "email", "uuid", and so on,
    // > MAY be used even though undefined by this specification.
}

impl Display for FormatModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FormatModifier::Int32 => "int32",
            FormatModifier::Int64 => "int64",
            FormatModifier::Float => "float",
        };
        write!(f, "{str}")
    }
}
