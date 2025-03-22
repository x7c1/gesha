use crate::Output;
use crate::v3_0::PathItemObject;
use crate::v3_0::yaml_extractor::collect;
use crate::yaml::YamlMap;

#[allow(dead_code)]
#[derive(Debug)]
/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#pathsObject
pub struct PathsObject(Vec<(PathFieldName, PathItemObject)>);

impl PathsObject {
    /// > The Paths MAY be empty, due to ACL constraints.
    pub fn new(paths: Vec<(PathFieldName, PathItemObject)>) -> Self {
        // TODO: check if each PathFieldName is unique in paths
        PathsObject(paths)
    }

    pub fn from_yaml_map(map: YamlMap) -> Output<PathsObject> {
        collect(Output::by(PathItemObject::with_name))(map).map(PathsObject::new)
    }
}

/// e.g. /pets
#[allow(dead_code)]
#[derive(Debug)]
pub struct PathFieldName(String);

impl PathFieldName {
    /// > The field name MUST begin with a forward slash (/).
    pub fn new<A: Into<String>>(a: A) -> Self {
        // TODO: check field pattern
        PathFieldName(a.into())
    }
}
