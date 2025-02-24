use crate::yaml::{YamlArray, YamlMap};
use crate::Error::TypeMismatch;
use crate::{Error, Result};

#[derive(Clone, Debug)]
pub enum YamlValue {
    Array(YamlArray),
    Boolean(bool),
    String(String),
    Map(YamlMap),
}

impl YamlValue {
    pub fn kind(&self) -> &'static str {
        match self {
            YamlValue::Array(_) => "Array",
            YamlValue::Boolean(_) => "Boolean",
            YamlValue::String(_) => "String",
            YamlValue::Map(_) => "Map",
        }
    }
}

impl TryFrom<yaml_rust::Yaml> for YamlValue {
    type Error = Error;

    fn try_from(yaml: yaml_rust::Yaml) -> Result<Self> {
        match yaml {
            yaml_rust::Yaml::Array(x) => Ok(YamlValue::Array(YamlArray(x))),
            yaml_rust::Yaml::String(x) => Ok(YamlValue::String(x)),
            yaml_rust::Yaml::Hash(x) => Ok(YamlValue::Map(YamlMap(x))),
            yaml_rust::Yaml::Boolean(x) => Ok(YamlValue::Boolean(x)),
            unknown => Err(Error::UnknownDataType(format!("{unknown:?}"))),
        }
    }
}

impl TryFrom<YamlValue> for YamlArray {
    type Error = Error;

    fn try_from(value: YamlValue) -> Result<Self> {
        match value {
            YamlValue::Array(x) => Ok(x),
            _ => Err(TypeMismatch {
                expected: "Array".to_string(),
                found: value.kind().to_string(),
            }),
        }
    }
}

impl TryFrom<YamlValue> for String {
    type Error = Error;

    fn try_from(value: YamlValue) -> Result<Self> {
        match value {
            YamlValue::String(x) => Ok(x),
            _ => Err(TypeMismatch {
                expected: "String".to_string(),
                found: value.kind().to_string(),
            }),
        }
    }
}

impl TryFrom<YamlValue> for bool {
    type Error = Error;

    fn try_from(value: YamlValue) -> Result<Self> {
        match value {
            YamlValue::Boolean(x) => Ok(x),
            _ => Err(TypeMismatch {
                expected: "Boolean".to_string(),
                found: value.kind().to_string(),
            }),
        }
    }
}

impl TryFrom<YamlValue> for YamlMap {
    type Error = Error;

    fn try_from(value: YamlValue) -> Result<Self> {
        match value {
            YamlValue::Map(x) => Ok(x),
            _ => Err(TypeMismatch {
                expected: "Map".to_string(),
                found: value.kind().to_string(),
            }),
        }
    }
}
