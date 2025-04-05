use crate::v3_0::SpecViolation::DuplicatedPathFieldName;
use crate::v3_0::yaml_extractor::collect;
use crate::v3_0::{PathFieldName, PathItemObject};
use crate::{Error, Output};
use gesha_collections::seq::VecPairsOps;
use gesha_collections::yaml::YamlMap;

type Pair = (PathFieldName, PathItemObject);

#[allow(dead_code)]
#[derive(Debug)]
/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#paths-object
pub struct PathsObject(Vec<Pair>);

impl PathsObject {
    /// > The Paths MAY be empty, due to ACL constraints.
    pub fn new(paths: Vec<Pair>) -> Output<Self> {
        let (paths, errors) = dedup(paths);
        Output::ok(PathsObject(paths)).append(errors)
    }

    pub fn from_yaml_map(map: YamlMap) -> Output<PathsObject> {
        collect(Output::by(PathItemObject::with_name))(map)
            .map(PathsObject::new)
            .flatten()
    }
}

fn dedup(paths: Vec<Pair>) -> (Vec<Pair>, Vec<Error>) {
    let (paths, duplicated_names) = paths.partition_unique_by_key();
    let errors = if duplicated_names.is_empty() {
        vec![]
    } else {
        let err = DuplicatedPathFieldName {
            fields: duplicated_names.dedup_keys(),
        };
        vec![err.into()]
    };
    (paths, errors)
}
