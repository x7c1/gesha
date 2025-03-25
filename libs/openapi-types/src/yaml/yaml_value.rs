use crate::yaml::{YamlArray, YamlError, YamlMap};

#[derive(Clone, Debug)]
pub enum YamlValue {
    Array(YamlArray),
    Boolean(bool),
    Integer(i64),
    Map(YamlMap),
    Null,
    String(String),
}

impl YamlValue {
    pub fn kind(&self) -> &'static str {
        match self {
            YamlValue::Array(_) => "Array",
            YamlValue::Boolean(_) => "Boolean",
            YamlValue::Integer(_) => "Integer",
            YamlValue::Map(_) => "Map",
            YamlValue::Null => "Null",
            YamlValue::String(_) => "String",
        }
    }
    pub fn outline(&self) -> String {
        match self {
            YamlValue::Array(_) => "<array>".to_string(),
            YamlValue::Boolean(x) => x.to_string(),
            YamlValue::Integer(x) => x.to_string(),
            YamlValue::Map(_) => "<map>".to_string(),
            YamlValue::Null => "null".to_string(),
            YamlValue::String(x) => x.chars().take(50).collect(),
        }
    }
}

impl TryFrom<yaml_rust::Yaml> for YamlValue {
    type Error = YamlError;

    fn try_from(yaml: yaml_rust::Yaml) -> std::result::Result<Self, YamlError> {
        match yaml {
            yaml_rust::Yaml::Array(x) => Ok(YamlValue::Array(YamlArray(x))),
            yaml_rust::Yaml::String(x) => Ok(YamlValue::String(x)),
            yaml_rust::Yaml::Hash(x) => Ok(YamlValue::Map(YamlMap(x))),
            yaml_rust::Yaml::Boolean(x) => Ok(YamlValue::Boolean(x)),
            yaml_rust::Yaml::Integer(x) => Ok(YamlValue::Integer(x)),
            yaml_rust::Yaml::Null => Ok(YamlValue::Null),
            unknown => Err(YamlError::UnknownType {
                found: format!("{unknown:?}"),
            }),
        }
    }
}

impl TryFrom<YamlValue> for YamlArray {
    type Error = YamlError;

    fn try_from(value: YamlValue) -> std::result::Result<Self, YamlError> {
        match value {
            YamlValue::Array(x) => Ok(x),
            _ => Err(YamlError::TypeMismatch {
                expected: "Array".to_string(),
                found: value.kind().to_string(),
            }),
        }
    }
}

impl TryFrom<YamlValue> for String {
    type Error = YamlError;

    fn try_from(value: YamlValue) -> std::result::Result<Self, YamlError> {
        match value {
            YamlValue::String(x) => Ok(x),
            _ => Err(YamlError::TypeMismatch {
                expected: "String".to_string(),
                found: value.kind().to_string(),
            }),
        }
    }
}

impl TryFrom<YamlValue> for bool {
    type Error = YamlError;

    fn try_from(value: YamlValue) -> std::result::Result<Self, YamlError> {
        match value {
            YamlValue::Boolean(x) => Ok(x),
            _ => Err(YamlError::TypeMismatch {
                expected: "Boolean".to_string(),
                found: value.kind().to_string(),
            }),
        }
    }
}

impl TryFrom<YamlValue> for YamlMap {
    type Error = YamlError;

    fn try_from(value: YamlValue) -> std::result::Result<Self, YamlError> {
        match value {
            YamlValue::Map(x) => Ok(x),
            _ => Err(YamlError::TypeMismatch {
                expected: "Map".to_string(),
                found: value.kind().to_string(),
            }),
        }
    }
}
