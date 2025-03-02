use crate::v3_0::components::schemas::{StructShape, TypeHeaderShape};
use gesha_core::conversions::Result;

#[derive(Clone, Debug)]
pub struct InlineStructShape {}

impl InlineStructShape {
    pub fn expand_with(&self, _shape: TypeHeaderShape) -> Result<StructShape> {
        todo!()
    }
}
