use crate::conversions::v3_0::to_rust_type::components::schemas::{
    AllOfItemShape, AllOfShape, FieldShape, ModShape, StructShape, TypeHeaderShape, TypeShape,
};
use openapi_types::v3_0::{ComponentName, EnumValues};

#[derive(Clone, Debug)]
pub enum DefinitionShape {
    AllOf(AllOfShape),
    Struct(StructShape),
    NewType {
        header: TypeHeaderShape,
        type_shape: TypeShape,
    },
    Enum {
        header: TypeHeaderShape,
        values: EnumValues,
    },
    Mod(ModShape),
}

impl DefinitionShape {
    pub fn as_type_definition(&self) -> Option<TypeDefinitionShape> {
        match self {
            DefinitionShape::Struct(shape) => Some(TypeDefinitionShape {
                type_header: &shape.header,
                fields: &shape.fields,
            }),
            DefinitionShape::AllOf { .. } // TODO: return here to merge multiple allOf
            | DefinitionShape::NewType { .. }
            | DefinitionShape::Enum { .. }
            | DefinitionShape::Mod { .. } => None,
        }
    }

    pub fn any_type(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        match self {
            DefinitionShape::AllOf(x) => x.any_type(f),
            DefinitionShape::Struct(x) => FieldShape::any_type(&x.fields, f),
            DefinitionShape::NewType { type_shape, .. } => f(type_shape),
            DefinitionShape::Enum { .. } => false,
            DefinitionShape::Mod(x) => x.defs.iter().any(|x| x.any_type(f)),
        }
    }
}

pub struct TypeDefinitionShape<'a> {
    type_header: &'a TypeHeaderShape,
    fields: &'a Vec<FieldShape>,
}

impl TypeDefinitionShape<'_> {
    pub fn type_name(&self) -> &ComponentName {
        &self.type_header.name
    }

    pub fn is_type_name(&self, name: &str) -> bool {
        self.type_name().as_ref() == name
    }

    pub fn is_nullable(&self) -> bool {
        self.type_header.is_nullable
    }

    pub fn field_shapes(&self) -> &[FieldShape] {
        self.fields
    }
}
