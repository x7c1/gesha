use crate::Result;
use crate::json_schema::SpecViolation::EmptyRequired;
use crate::v3_0::yaml_map_ext::reify_value;
use gesha_collections::yaml::YamlArray;
use indexmap::IndexSet;

/// https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.15
/// >The value of this keyword MUST be an array.  This array MUST have at
/// >least one element. Elements of this array MUST be strings, and MUST
/// >be unique.
#[derive(Clone, Debug)]
pub struct RequiredSchemaFields(IndexSet<String>);

impl RequiredSchemaFields {
    pub fn new(fields: IndexSet<String>) -> Result<Self> {
        if fields.is_empty() {
            Err(EmptyRequired)?;
        }
        Ok(Self(fields))
    }

    pub fn contains(&self, field_name: &str) -> bool {
        self.0.contains(field_name)
    }

    pub(crate) fn from_yaml_array(array: YamlArray) -> Result<RequiredSchemaFields> {
        let fields = array
            .into_iter()
            .map(reify_value)
            .collect::<Result<IndexSet<String>>>()?;

        RequiredSchemaFields::new(fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Error, SpecViolation};

    #[test]
    fn err_empty_set() {
        let set = IndexSet::new();
        let err = RequiredSchemaFields::new(set).unwrap_err();
        let violation = spec_violation(err);
        assert_eq!(violation, EmptyRequired.into());
    }

    fn spec_violation(e: Error) -> SpecViolation {
        match e {
            Error::SpecViolation(violation) => violation,
            _ => panic!("unexpected error: {:?}", e),
        }
    }
}
