use crate::v3_0::components::schemas::{AllOfShape, TypeHeaderShape};
use gesha_core::conversions::Result;

#[derive(Clone, Debug)]
pub struct InlineAllOfShape {}

impl InlineAllOfShape {
    pub fn expand_with(&self, _shape: TypeHeaderShape) -> Result<AllOfShape> {
        todo!()
    }
}
