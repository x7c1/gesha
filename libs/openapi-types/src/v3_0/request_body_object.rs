use crate::v3_0::{ReferenceObject, RequestBodyContent};

/// Request Body Object | Reference Object
#[derive(Clone, Debug)]
pub enum RequestBodyCase {
    RequestBody(Box<RequestBodyObject>),
    Reference(ReferenceObject<RequestBodyObject>),
}

/// rf. https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#requestBodyObject
#[derive(Clone, Debug)]
pub struct RequestBodyObject {
    pub description: Option<String>,
    pub content: RequestBodyContent,
    pub required: bool,
}
