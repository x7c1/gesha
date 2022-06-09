mod operation_object;
pub use operation_object::{
    HttpStatusCode, OperationObject, ResponseCase, ResponseObject, ResponsesObject,
};

mod paths_object;
pub use paths_object::{PathFieldName, PathItemObject, PathsObject};

mod components_object;
pub use components_object::{
    AllOf, ArrayItems, ComponentsObject, EnumValues, RequiredSchemaFields, SchemaCase,
    SchemaFieldName, SchemaObject, SchemaProperties, SchemasObject,
};

mod format_modifier;
pub use format_modifier::FormatModifier;

mod openapi_data_type;
pub use openapi_data_type::OpenApiDataType;

mod reference_object;
pub use reference_object::ReferenceObject;

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
