use crate::v3_0::components::schemas::Ref;
use gesha_core::conversion::Result;
use openapi_types::v3_0::SchemaCase;

#[derive(Clone, Debug)]
pub struct OneOfItemShape {
    pub target: Ref,
}

impl OneOfItemShape {
    pub fn from_schema_cases(cases: Vec<SchemaCase>) -> Result<Vec<Self>> {
        cases.into_iter().map(Self::from_schema_case).collect()
    }

    fn from_schema_case(case: SchemaCase) -> Result<Self> {
        let shape = match case {
            SchemaCase::Schema(_) => unimplemented!("not supported"),
            SchemaCase::Reference(target) => Self { target },
        };
        Ok(shape)
    }
}
