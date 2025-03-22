use crate::core::OutputOptionOps;
use crate::json_schema::SpecViolation::EmptyOneOf;
use crate::v3_0::SchemaCase;
use crate::yaml::YamlArray;
use crate::{Output, Result};

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#schema-object
/// > Inline or referenced schema MUST be of a Schema Object and not a standard JSON Schema.
///
/// https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.24
/// > This keyword's value MUST be an array.  This array MUST have at least one element.
///
/// > Elements of the array MUST be objects.  Each object MUST be a valid JSON Schema.
#[derive(Clone, Debug)]
pub struct OneOf(Vec<SchemaCase>);

impl OneOf {
    pub fn new(cases: Vec<SchemaCase>) -> Result<Self> {
        if cases.is_empty() {
            Err(EmptyOneOf)?;
        }
        Ok(Self(cases))
    }

    pub fn from_yaml_array(array: YamlArray) -> Output<Option<Self>> {
        SchemaCase::from_yaml_array(array).map(Self::new).maybe()
    }
}

impl From<OneOf> for Vec<SchemaCase> {
    fn from(this: OneOf) -> Self {
        this.0
    }
}
