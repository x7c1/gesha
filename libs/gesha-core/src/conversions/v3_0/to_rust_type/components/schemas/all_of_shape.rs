use crate::conversions::v3_0::to_rust_type::components::schemas::{
    AllOfItemShape, DefinitionShape, FieldShape, TypeHeaderShape, TypeShape,
};

#[derive(Clone, Debug)]
pub struct AllOfShape {
    pub header: TypeHeaderShape,
    pub items: Vec<AllOfItemShape>,
}

impl AllOfShape {
    pub fn any_type(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        self.items.iter().any(|item| match item {
            AllOfItemShape::Object(xs) => FieldShape::any_type(xs, f),
            AllOfItemShape::Ref(_) => false,
        })
    }

    pub fn any_type_directly(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        self.items.iter().any(|item| match item {
            AllOfItemShape::Object(xs) => xs.iter().any(|x| f(&x.type_shape)),
            AllOfItemShape::Ref(_) => false,
        })
    }
}

impl From<AllOfShape> for DefinitionShape {
    fn from(this: AllOfShape) -> Self {
        Self::AllOf(this)
    }
}
