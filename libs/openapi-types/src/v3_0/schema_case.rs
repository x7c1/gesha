use crate::v3_0::yaml_extractor::reify_value;
use crate::v3_0::{ComponentName, ReferenceObject, SchemaObject};
use crate::{Output, Result, by_key};
use gesha_collections::partial_result::MergeOps;
use gesha_collections::yaml::{YamlArray, YamlMap, YamlMapExt};

pub type NamedSchemaCase = (ComponentName, SchemaCase);

/// Schema Object | Reference Object
#[derive(Clone, Debug)]
pub enum SchemaCase {
    Schema(Box<SchemaObject>),
    Reference(ReferenceObject<SchemaObject>),
}

impl SchemaCase {
    pub fn from_yaml_map(mut map: YamlMap) -> Result<SchemaCase> {
        let case = match map.extract_if_exists("$ref", Output::ok).to_result()? {
            Some(rf) => {
                let reference = ReferenceObject::new::<String>(rf);
                SchemaCase::Reference(reference)
            }
            None => {
                let object = SchemaObject::from_yaml_map(map)?;
                SchemaCase::Schema(Box::new(object))
            }
        };
        Ok(case)
    }

    pub fn from_yaml_array(array: YamlArray) -> Output<Vec<SchemaCase>> {
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

    pub fn with_name(kv: (String, YamlMap)) -> Result<NamedSchemaCase> {
        let (name, map) = kv;
        let pair = (
            ComponentName::new(&name),
            Self::from_yaml_map(map).map_err(by_key(name))?,
        );
        Ok(pair)
    }
}
