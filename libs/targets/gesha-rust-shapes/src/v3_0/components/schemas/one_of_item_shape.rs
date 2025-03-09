use crate::v3_0::components::schemas::{CaseItem, CaseItemShapes, RefShape};
use gesha_core::conversions::{Output, Result};
use openapi_types::v3_0::SchemaCase;

pub type OneOfItemShapes = CaseItemShapes<OneOfItemShape>;

#[derive(Clone, Debug)]
pub struct OneOfItemShape {
    pub target: RefShape,
}

impl OneOfItemShape {
    pub fn from_schema_case(case: SchemaCase) -> Result<Output<Self>> {
        let shape = match case {
            SchemaCase::Schema(_) => unimplemented!("not supported"),
            SchemaCase::Reference(target) => {
                let target = RefShape::new(target, /* is_required */ true)?;
                Self { target }
            }
        };
        Ok(Output::ok(shape))
    }
}

impl CaseItem for OneOfItemShape {
    fn from_schema_case(case: SchemaCase) -> Result<Output<Self>> {
        Self::from_schema_case(case)
    }

    fn to_ref_shape(&self) -> Option<&RefShape> {
        Some(&self.target)
    }
}
