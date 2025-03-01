use crate::v3_0::components::schemas::{
    DefinitionShape, FieldShape, OneOfItemShape, RefShape, TypeHeaderShape, TypeShape,
};

#[derive(Clone, Debug)]
pub struct OneOfShape {
    pub header: TypeHeaderShape,
    pub items: Vec<OneOfItemShape>,
}

impl OneOfShape {
    pub fn any_type(&self, _: &impl Fn(&TypeShape) -> bool) -> bool {
        // implement when inline item is supported
        false
    }

    pub fn any_type_directly(&self, _: &impl Fn(&TypeShape) -> bool) -> bool {
        // implement when inline item is supported
        false
    }

    pub fn expand_fields(&self, _: impl Fn(&RefShape) -> Vec<FieldShape>) -> Vec<FieldShape> {
        // implement when inline item is supported
        vec![]
    }
}

impl From<OneOfShape> for DefinitionShape {
    fn from(this: OneOfShape) -> Self {
        Self::OneOf(this)
    }
}
