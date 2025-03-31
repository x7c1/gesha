use crate::Output;
use crate::v3_0::SpecViolation::DuplicatedPathFieldName;
use crate::v3_0::yaml_map_ext::collect;
use crate::v3_0::{PathFieldName, PathItemObject};
use crate::yaml::YamlMap;
use gesha_collections::seq::VecPairs;

#[allow(dead_code)]
#[derive(Debug)]
/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#paths-object
pub struct PathsObject(Vec<(PathFieldName, PathItemObject)>);

impl PathsObject {
    /// > The Paths MAY be empty, due to ACL constraints.
    pub fn new(paths: Vec<(PathFieldName, PathItemObject)>) -> Output<Self> {
        let (paths, duplicated_names) = paths.partition_dedup_by_key();
        let errors = if duplicated_names.is_empty() {
            vec![]
        } else {
            let err = DuplicatedPathFieldName {
                fields: duplicated_names.dedup_keys(),
            };
            vec![err.into()]
        };
        Output::ok(PathsObject(paths)).append(errors)
    }

    pub fn from_yaml_map(map: YamlMap) -> Output<PathsObject> {
        collect(Output::by(PathItemObject::with_name))(map)
            .map(PathsObject::new)
            .flatten()
    }
}
