use crate::Error;
use crate::Error::IncompatibleType;

#[derive(Debug)]
pub enum YamlValue {
    String(String),
    Map(YamlMap),
}

impl TryFrom<yaml_rust::Yaml> for YamlValue {
    type Error = crate::Error;

    fn try_from(yaml: yaml_rust::Yaml) -> Result<Self, Self::Error> {
        match yaml {
            yaml_rust::Yaml::String(x) => Ok(YamlValue::String(x)),
            yaml_rust::Yaml::Hash(x) => Ok(YamlValue::Map(YamlMap(x))),
            unknown => return Err(Error::todo(format!("unsupported type found: {unknown:?}"))),
        }
    }
}

impl TryFrom<YamlValue> for String {
    type Error = crate::Error;

    fn try_from(value: YamlValue) -> Result<Self, Self::Error> {
        match value {
            YamlValue::String(x) => Ok(x),
            _ => Err(IncompatibleType),
        }
    }
}

impl TryFrom<YamlValue> for YamlMap {
    type Error = crate::Error;

    fn try_from(value: YamlValue) -> Result<Self, Self::Error> {
        match value {
            YamlValue::Map(x) => Ok(x),
            _ => Err(IncompatibleType),
        }
    }
}

#[derive(Debug)]
pub struct YamlMap(yaml_rust::yaml::Hash);

impl YamlMap {
    pub fn remove(&mut self, key: &str) -> crate::Result<YamlValue> {
        // TODO: remove unwrap
        let x = self.0.remove(&yaml_rust::Yaml::from_str(key)).unwrap();
        x.try_into()
    }
}
