use crate::v3_0::components::schemas::RefShape;
use gesha_core::conversions::Result;
use openapi_types::v3_0::SchemaCase;

#[derive(Clone, Debug)]
pub struct OneOfItemShape {
    pub target: RefShape,
}

impl OneOfItemShape {
    pub fn from_schema_case(case: SchemaCase) -> Result<Self> {
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
