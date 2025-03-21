use crate::core::OutputMergeOps;
use crate::v3_0::{ComponentName, SchemaCase};
use crate::yaml::{YamlArray, YamlMap, reify_value};
use crate::{Output, Result, by_key};

pub fn to_schema_pair(kv: (String, YamlMap)) -> Result<(ComponentName, SchemaCase)> {
    let (name, map) = kv;
    let pair = (
        ComponentName::new(&name),
        SchemaCase::from_yaml_map(map).map_err(by_key(name))?,
    );
    Ok(pair)
}

pub fn to_schema_cases(array: YamlArray) -> Output<Vec<SchemaCase>> {
    array
        .into_iter()
        .map(reify_value)
        .collect::<Vec<Result<YamlMap>>>()
        .merge()
        .map(|xs| {
            xs.into_iter()
                .map(SchemaCase::from_yaml_map)
                .collect::<Result<Vec<SchemaCase>>>()
                .merge()
        })
        .merge()
}
