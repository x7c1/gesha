use crate::conversions::v3_0::to_rust_type::{AllOfItemShape, FieldShape, TypeShape};
use crate::targets::rust_type::TypeHeader;
use openapi_types::v3_0::EnumValues;

#[derive(Clone, Debug)]
pub(super) enum DefinitionShape {
    AllOf {
        header: TypeHeader,
        shapes: Vec<AllOfItemShape>,
    },
    Struct {
        header: TypeHeader,
        shapes: Vec<FieldShape>,
    },
    NewType {
        header: TypeHeader,
        type_shape: TypeShape,
    },
    Enum {
        header: TypeHeader,
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
        header.name == name
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
