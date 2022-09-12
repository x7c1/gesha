use crate::v3_0::{ComponentName, ReferenceObject, RequestBodyContent};
use indexmap::IndexMap;

type InnerMap = IndexMap<ComponentName, RequestBodyCase>;

/// rf. https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#componentsObject
#[derive(Debug)]
pub struct RequestBodiesObject(InnerMap);

impl RequestBodiesObject {
    pub fn new(map: InnerMap) -> Self {
        Self(map)
    }
}

impl IntoIterator for RequestBodiesObject {
    type Item = (ComponentName, RequestBodyCase);
    type IntoIter = <InnerMap as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
pub struct RequestBodyObject {
    pub description: Option<String>,
    pub content: RequestBodyContent,
    pub required: bool,
}
