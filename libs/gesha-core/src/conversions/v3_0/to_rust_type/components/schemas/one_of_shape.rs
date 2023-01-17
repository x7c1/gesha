use crate::conversions::v3_0::to_rust_type::components::schemas::DefinitionShape;

#[derive(Clone, Debug)]
pub struct OneOfShape {}

impl From<OneOfShape> for DefinitionShape {
    fn from(this: OneOfShape) -> Self {
        Self::OneOf(this)
    }
}
