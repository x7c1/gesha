use crate::Result;
use crate::v3_0::SchemaCase;
use crate::v3_0::from_yaml::to_schema_case;
use crate::yaml::YamlMap;

/// > Value MUST be an object and not an array.
/// > Inline or referenced schema MUST be of a Schema Object and
/// > not a standard JSON Schema. items MUST be present if the type is array.
///
/// ---
///
/// see also: https://swagger.io/docs/specification/data-models/data-types/
///
#[derive(Clone, Debug)]
pub struct ArrayItems(Box<SchemaCase>);

impl ArrayItems {
    pub fn new(case: SchemaCase) -> Self {
        Self(Box::new(case))
    }

    pub fn from_yaml_map(map: YamlMap) -> Result<Self> {
        let case = to_schema_case(map)?;
        let items = ArrayItems::new(case);
        Ok(items)
    }
}

impl From<ArrayItems> for SchemaCase {
    fn from(xs: ArrayItems) -> Self {
        *xs.0
    }
}
