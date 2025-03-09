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

    pub fn pop_if_only_one_ref(&self) -> Option<RefShape> {
        let ref_shape = match self.items.as_slice() {
            [OneOfItemShape { target }] => Some(target.clone()),
            _ => None,
        };
        let mut ref_shape = ref_shape?;
        ref_shape.nullable = Some(self.header.is_nullable);
        Some(ref_shape)
    }
}

impl From<OneOfShape> for DefinitionShape {
    fn from(this: OneOfShape) -> Self {
        Self::OneOf(this)
    }
}
