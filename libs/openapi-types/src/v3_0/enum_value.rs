use crate::yaml::YamlValue;
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

impl TryFrom<YamlValue> for EnumValue {
    type Error = crate::Error;

    fn try_from(value: YamlValue) -> Result<Self, Self::Error> {
        let this = match value {
            YamlValue::String(x) => Self::String(x),
            YamlValue::Integer(x) => Self::Integer(x),
            YamlValue::Boolean(x) => Self::Boolean(x),
            YamlValue::Array(_) => {
                return Err(crate::Error::UnknownDataType {
                    found: "<array>".to_string(),
                })
            }
            YamlValue::Map(_) => {
                return Err(crate::Error::UnknownDataType {
                    found: "<object>".to_string(),
                })
            }
        };
        Ok(this)
    }
}
