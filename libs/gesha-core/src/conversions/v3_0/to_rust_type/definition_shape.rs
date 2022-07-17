use crate::conversions::v3_0::to_rust_type::{
    AllOfItemShape, FieldShape, TypeHeaderShape, TypeShape,
};
use openapi_types::v3_0::EnumValues;

#[derive(Clone, Debug)]
pub(super) enum DefinitionShape {
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
    // TODO: rename -> is_type_name
    pub fn is_struct_name(&self, name: &str) -> bool {
        let header = match self {
            DefinitionShape::AllOf { header, .. } => header,
            DefinitionShape::Struct { header, .. } => header,
            DefinitionShape::NewType { header, .. } => header,
            DefinitionShape::Enum { header, .. } => header,
        };
        header.name.as_ref() == name
    }

    pub fn is_nullable(&self) -> bool {
        // TODO:
        false
    }

    pub fn field_shapes(&self) -> Vec<FieldShape> {
        match self {
            DefinitionShape::Struct { shapes, .. } => shapes.clone(),
            DefinitionShape::AllOf { .. } => unimplemented!(),
            DefinitionShape::NewType { .. } => unimplemented!(),
            DefinitionShape::Enum { .. } => unimplemented!(),
        }
    }
}
