use crate::conversions::v3_0::to_rust_type::components::schemas::Ref;
use crate::conversions::Result;
use openapi_types::v3_0::SchemaCase;

#[derive(Clone, Debug)]
pub struct OneOfItemShape {
    target: Ref,
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
