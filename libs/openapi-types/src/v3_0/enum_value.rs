use crate::yaml::{YamlArray, YamlValue, reify_value};
use crate::{Error, Result};
use indexmap::IndexSet;

/// https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.20
/// > The value of this keyword MUST be an array. This array SHOULD have
/// > at least one element.  Elements in the array SHOULD be unique.
/// > Elements in the array MAY be of any type, including null.
pub type EnumValues = IndexSet<EnumValue>;

/**
    OpenAPI allows "any type", but this library does not support some of them.
    Below are the unsupported cases and the reasons:

    - [float] : equality is ambiguous
    - [object, array] : variant name cannot be determined
*/
#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum EnumValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    Null,
}

impl EnumValue {
    pub fn from_yaml_array(array: YamlArray) -> Result<EnumValues> {
        array.into_iter().map(reify_value).collect()
    }
}

impl TryFrom<YamlValue> for EnumValue {
    type Error = Error;

    fn try_from(value: YamlValue) -> Result<Self> {
        let this = match value {
            YamlValue::String(x) => Self::String(x),
            YamlValue::Integer(x) => Self::Integer(x),
            YamlValue::Boolean(x) => Self::Boolean(x),
            YamlValue::Array(_) => Err(unsupported("<array>"))?,
            YamlValue::Map(_) => Err(unsupported("<object>"))?,
        };
        Ok(this)
    }
}

fn unsupported(found: impl Into<String>) -> Error {
    Error::UnsupportedEnumType {
        expected: "<string>|<integer>|<boolean>|<null>".to_string(),
        found: found.into(),
    }
}
