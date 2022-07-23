use crate::v3_0::{ReferenceObject, SchemaFieldName};
use indexmap::IndexMap;

#[derive(Debug)]
pub struct RequestBodiesObject(IndexMap<SchemaFieldName, RequestBodyCase>);

impl RequestBodiesObject {
    pub fn new(map: IndexMap<SchemaFieldName, RequestBodyCase>) -> Self {
        Self(map)
    }
}

/// Request Body Object | Reference Object
#[derive(Clone, Debug)]
pub enum RequestBodyCase {
    RequestBody(Box<RequestBodyObject>),
    Reference(ReferenceObject),
}

/// rf. https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#requestBodyObject
#[derive(Clone, Debug)]
pub struct RequestBodyObject {}
