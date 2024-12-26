use crate::v3_0::components::schemas::{
    DefinitionShape, FieldShape, TypeHeaderShape, TypeShape,
};

#[derive(Clone, Debug)]
pub struct StructShape {
    pub header: TypeHeaderShape,
    pub fields: Vec<FieldShape>,
}

impl StructShape {
    pub fn any_type(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        FieldShape::any_type(&self.fields, f)
    }
    pub fn any_type_directly(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        self.fields.iter().any(|x| f(&x.type_shape))
    }
}

impl From<StructShape> for DefinitionShape {
    fn from(this: StructShape) -> Self {
        Self::Struct(this)
    }
}
