use crate::core::OutputMergeOps;
use crate::v3_0::{ReferenceObject, SchemaObject};
use crate::yaml::{YamlArray, YamlMap, reify_value};
use crate::{Output, Result};

/// Schema Object | Reference Object
#[derive(Clone, Debug)]
pub enum SchemaCase {
    Schema(Box<SchemaObject>),
    Reference(ReferenceObject<SchemaObject>),
}

impl SchemaCase {
    pub fn from_yaml_map(mut map: YamlMap) -> Result<SchemaCase> {
        let case = match map.remove_if_exists::<String>("$ref")? {
            Some(rf) => {
                let reference = ReferenceObject::new(rf);
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
}
