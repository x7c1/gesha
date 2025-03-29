use crate::Output;
use crate::v3_0::yaml_extractor::collect;
use crate::v3_0::{PathFieldName, PathItemObject};
use crate::yaml::YamlMap;
use indexmap::{IndexMap, IndexSet};
use std::hash::Hash;

#[allow(dead_code)]
#[derive(Debug)]
/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.4.md#paths-object
pub struct PathsObject(Vec<(PathFieldName, PathItemObject)>);

impl PathsObject {
    /// > The Paths MAY be empty, due to ACL constraints.
    pub fn new(paths: Vec<(PathFieldName, PathItemObject)>) -> Output<Self> {
        let (paths, duplicated_names) = dedup_by_key(paths);
        println!("duplicated_names: {duplicated_names:#?}");

        // TODO contain `duplicated` as error

        Output::ok(PathsObject(paths))
    }

    pub fn from_yaml_map(map: YamlMap) -> Output<PathsObject> {
        collect(Output::by(PathItemObject::with_name))(map)
            .map(PathsObject::new)
            .flatten()
    }
}

/// Deduplicate key-value pairs by key.
///
/// Returns a list of unique pairs and a list of duplicated keys.
/// If multiple pairs have the same key, the first occurrence is kept.
fn dedup_by_key<K, V>(pairs: Vec<(K, V)>) -> (Vec<(K, V)>, Vec<K>)
where
    K: PartialEq + Eq + Hash + Clone,
{
    let separate = |(mut map, mut set): (IndexMap<K, V>, IndexSet<K>), (key, value)| {
        if map.get(&key).is_none() {
            map.insert(key, value);
        } else {
            set.insert(key);
        }
        (map, set)
    };
    let (unique_map, duplicated_set) = pairs.into_iter().fold(Default::default(), separate);
    let unique = unique_map.into_iter().collect();
    let duplicated = duplicated_set.into_iter().collect();
    (unique, duplicated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn err_paths_object() -> Result<()> {
        let params = vec!["/pets", "/pets"]
            .into_iter()
            .map(String::from)
            .map(PathFieldName::new)
            .map(|name| {
                let object = PathItemObject::from_yaml_map(YamlMap::default());
                Ok((name?, object?))
            })
            .collect::<Result<Vec<_>>>()?;

        let error = PathsObject::new(params).into_tuple();
        println!("error ... {error:#?}");
        Ok(())
    }
}
