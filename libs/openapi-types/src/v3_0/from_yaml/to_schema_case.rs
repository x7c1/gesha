use crate::v3_0::{ComponentName, SchemaCase};
use crate::yaml::YamlMap;
use crate::{Result, by_key};

pub fn to_schema_pair(kv: (String, YamlMap)) -> Result<(ComponentName, SchemaCase)> {
    let (name, map) = kv;
    let pair = (
        ComponentName::new(&name),
        SchemaCase::from_yaml_map(map).map_err(by_key(name))?,
    );
    Ok(pair)
}
