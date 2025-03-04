use crate::v3_0::components::schemas::RefShape;
use gesha_core::conversions::{Output, Result};
use openapi_types::core::OutputMergeOps;
use openapi_types::v3_0::SchemaCase;

#[derive(Clone, Debug)]
pub struct OneOfItemShape {
    pub target: RefShape,
}

impl OneOfItemShape {
    pub fn from_schema_cases(cases: Vec<SchemaCase>) -> Output<Vec<Self>> {
        cases
            .into_iter()
            .map(Self::from_schema_case)
            .collect::<Vec<_>>()
            .merge()
    }

    fn from_schema_case(case: SchemaCase) -> Result<Self> {
        let shape = match case {
            SchemaCase::Schema(_) => unimplemented!("not supported"),
            SchemaCase::Reference(target) => {
                let target = RefShape::new(target, /* is_required */ true)?;
                Self { target }
            }
        };
        Ok(shape)
    }
}
