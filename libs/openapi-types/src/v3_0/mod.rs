mod operation_object;
pub use operation_object::{
    HttpStatusCode, OperationObject, ReferenceObject, ResponseCase, ResponseObject, ResponsesObject,
};

mod paths_object;
pub use paths_object::{PathFieldName, PathItemObject, PathsObject};

/// OpenAPI Document
/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#schema
#[derive(Debug)]
pub struct Document {
    pub openapi: String,
    pub info: InfoObject,
    pub paths: PathsObject,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#infoObject
#[derive(Debug)]
pub struct InfoObject {
    pub title: String,
}
