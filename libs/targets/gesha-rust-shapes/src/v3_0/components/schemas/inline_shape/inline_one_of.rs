use crate::v3_0::components::schemas::{OneOfShape, TypeHeaderShape};
use gesha_core::conversions::Result;

#[derive(Clone, Debug)]
pub struct InlineOneOfShape {}

impl InlineOneOfShape {
    pub fn expand_with(&self, _shape: TypeHeaderShape) -> Result<OneOfShape> {
        todo!()
    }
}
