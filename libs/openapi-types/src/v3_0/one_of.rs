use crate::core::OutputOptionOps;
use crate::v3_0::SchemaCase;
use crate::v3_0::from_yaml::to_schema_cases;
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
    pub fn new(xs: Vec<SchemaCase>) -> Result<Self> {
        // TODO: check xs length
        Ok(Self(xs))
    }

    pub fn from_yaml_array(array: YamlArray) -> Output<Option<Self>> {
        to_schema_cases(array).map(Self::new).maybe()
    }
}

impl From<OneOf> for Vec<SchemaCase> {
    fn from(xs: OneOf) -> Self {
        xs.0
    }
}
