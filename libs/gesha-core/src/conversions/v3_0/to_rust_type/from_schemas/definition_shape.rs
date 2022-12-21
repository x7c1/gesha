use crate::conversions::v3_0::to_rust_type::from_schemas::{
    AllOfItemShape, FieldShape, TypeHeaderShape, TypeShape,
};
use openapi_types::v3_0::{ComponentName, EnumValues};

#[derive(Clone, Debug)]
pub enum DefinitionShape {
    AllOf {
        header: TypeHeaderShape,
        shapes: Vec<AllOfItemShape>,
    },
    Struct {
        header: TypeHeaderShape,
        shapes: Vec<FieldShape>,
    },
    NewType {
        header: TypeHeaderShape,
        type_shape: TypeShape,
    },
    Enum {
        header: TypeHeaderShape,
        values: EnumValues,
    },
}

impl DefinitionShape {
    pub fn type_name(&self) -> &ComponentName {
        &self.type_header().name
    }

    pub fn is_type_name(&self, name: &str) -> bool {
        self.type_name().as_ref() == name
    }

    pub fn is_nullable(&self) -> bool {
        self.type_header().is_nullable
    }

    pub fn field_shapes(&self) -> Vec<FieldShape> {
        match self {
            DefinitionShape::Struct { shapes, .. } => shapes.clone(),
            DefinitionShape::AllOf { .. } => unimplemented!(),
            DefinitionShape::NewType { .. } => unimplemented!(),
            DefinitionShape::Enum { .. } => unimplemented!(),
        }
    }

    pub fn has_inline_schemas(&self) -> bool {
        match self {
            DefinitionShape::Struct { shapes, .. } => shapes
                .iter()
                .any(|x| matches!(x.type_shape, TypeShape::InlineObject { .. })),
            DefinitionShape::AllOf { .. } => false,
            DefinitionShape::NewType { .. } => false,
            DefinitionShape::Enum { .. } => false,
        }
    }

    fn type_header(&self) -> &TypeHeaderShape {
        match self {
            DefinitionShape::AllOf { header, .. } => header,
            DefinitionShape::Struct { header, .. } => header,
            DefinitionShape::NewType { header, .. } => header,
            DefinitionShape::Enum { header, .. } => header,
        }
    }
}
