use crate::yaml_wrapper::{YamlArray, YamlMap};
use crate::Error::TypeMismatch;

#[derive(Debug)]
pub enum YamlValue {
    Array(YamlArray),
    String(String),
    Map(YamlMap),
}

impl TryFrom<yaml_rust::Yaml> for YamlValue {
    type Error = crate::Error;

    fn try_from(yaml: yaml_rust::Yaml) -> Result<Self, Self::Error> {
        match yaml {
            yaml_rust::Yaml::Array(x) => Ok(YamlValue::Array(YamlArray(x))),
            yaml_rust::Yaml::String(x) => Ok(YamlValue::String(x)),
            yaml_rust::Yaml::Hash(x) => Ok(YamlValue::Map(YamlMap(x))),
            unknown => {
                return Err(crate::Error::todo(format!(
                    "unsupported type found: {unknown:#?}"
                )))
            }
        }
    }
}

impl TryFrom<YamlValue> for YamlArray {
    type Error = crate::Error;

    fn try_from(value: YamlValue) -> Result<Self, Self::Error> {
        match value {
            YamlValue::Array(x) => Ok(x),
            _ => Err(TypeMismatch),
        }
    }
}

impl TryFrom<YamlValue> for String {
    type Error = crate::Error;

    fn try_from(value: YamlValue) -> Result<Self, Self::Error> {
        match value {
            YamlValue::String(x) => Ok(x),
            _ => Err(TypeMismatch),
        }
    }
}

impl TryFrom<YamlValue> for YamlMap {
    type Error = crate::Error;

    fn try_from(value: YamlValue) -> Result<Self, Self::Error> {
        match value {
            YamlValue::Map(x) => Ok(x),
            _ => Err(TypeMismatch),
        }
    }
}
