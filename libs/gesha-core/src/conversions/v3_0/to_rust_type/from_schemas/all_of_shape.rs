use crate::conversions::v3_0::to_rust_type::from_schemas::{
    AllOfItemShape, DefinitionShape, FieldShape, TypeHeaderShape,
};

#[derive(Clone, Debug)]
pub struct AllOfShape {
    pub header: TypeHeaderShape,
    pub items: Vec<AllOfItemShape>,
}

impl From<AllOfShape> for DefinitionShape {
    fn from(this: AllOfShape) -> Self {
        Self::AllOf(this)
    }
}

impl AllOfShape {
    pub fn fields(&mut self) -> impl Iterator<Item = &mut FieldShape> {
        self.items
            .iter_mut()
            .filter_map(|x| match x {
                AllOfItemShape::Object(xs) => Some(xs),
                AllOfItemShape::Ref(_) => None,
            })
            .flatten()
    }
}
