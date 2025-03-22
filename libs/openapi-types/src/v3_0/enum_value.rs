use crate::Result;
use crate::v3_0::yaml_extractor::reify_value;
use crate::yaml::{YamlArray, YamlError, YamlValue};
use indexmap::IndexSet;

/// https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.20
/// > The value of this keyword MUST be an array. This array SHOULD have
/// > at least one element.  Elements in the array SHOULD be unique.
/// > Elements in the array MAY be of any type, including null.
#[derive(Clone, Debug, Default)]
pub struct EnumValues(IndexSet<EnumValue>);

impl EnumValues {
    pub fn from_yaml_array(array: YamlArray) -> Result<Self> {
        // No need to check for an empty array because the specs say it SHOULD, not MUST.
        let values = array.into_iter().map(reify_value).collect::<Result<_>>()?;
        Ok(EnumValues(values))
    }
}

impl IntoIterator for EnumValues {
    type Item = EnumValue;
    type IntoIter = <IndexSet<EnumValue> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

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

impl TryFrom<YamlValue> for EnumValue {
    type Error = YamlError;

    fn try_from(value: YamlValue) -> std::result::Result<Self, YamlError> {
        let this = match value {
            YamlValue::String(x) => Self::String(x),
            YamlValue::Integer(x) => Self::Integer(x),
            YamlValue::Boolean(x) => Self::Boolean(x),
            YamlValue::Array(_) => Err(YamlError::UnknownType {
                found: "<array>".to_string(),
            })?,
            YamlValue::Map(_) => Err(YamlError::UnknownType {
                found: "<object>".to_string(),
            })?,
        };
        Ok(this)
    }
}
