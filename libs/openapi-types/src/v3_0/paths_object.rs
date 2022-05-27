use crate::v3_0::OperationObject;

#[derive(Debug)]
/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#pathsObject
pub struct PathsObject(Vec<(PathFieldName, PathItemObject)>);

impl PathsObject {
    /// > The Paths MAY be empty, due to ACL constraints.
    pub fn new(paths: Vec<(PathFieldName, PathItemObject)>) -> Self {
        // TODO: check if each PathFieldName is unique in paths
        PathsObject(paths)
    }
}

/// e.g. /pets
#[derive(Debug)]
pub struct PathFieldName(String);

impl PathFieldName {
    /// > The field name MUST begin with a forward slash (/).
    pub fn new<A: Into<String>>(a: A) -> Self {
        // TODO: check field pattern
        PathFieldName(a.into())
    }
}

#[derive(Debug)]
pub struct PathItemObject {
    pub get: Option<OperationObject>,
    pub post: Option<OperationObject>,
}
