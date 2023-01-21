use crate::conversions::v3_0::to_rust_type::components::schemas::{
    DefinitionShape, OneOfItemShape, TypeHeaderShape, TypeShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::EnumDef;

#[derive(Clone, Debug)]
pub struct OneOfShape {
    pub header: TypeHeaderShape,
    pub items: Vec<OneOfItemShape>,
}

impl OneOfShape {
    pub fn any_type(&self, _: &impl Fn(&TypeShape) -> bool) -> bool {
        // return true when inline item is supported
        false
    }

    pub fn any_type_directly(&self, _: &impl Fn(&TypeShape) -> bool) -> bool {
        // return true when inline item is supported
        false
    }

    pub fn define(self) -> Result<EnumDef> {
        todo!("{:#?}", self)
    }
}

impl From<OneOfShape> for DefinitionShape {
    fn from(this: OneOfShape) -> Self {
        Self::OneOf(this)
    }
}
