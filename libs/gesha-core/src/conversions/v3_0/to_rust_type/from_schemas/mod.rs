mod definition_shape;
pub(super) use definition_shape::DefinitionShape;

mod to_shape;
use to_shape::to_shape;

mod to_field_shapes;

mod to_type_shape;
use to_type_shape::to_type_shape;

use crate::conversions::Result;
use crate::targets::rust_type::{DataType, DocComments};
use openapi_types::v3_0::{ComponentName, ReferenceObject, SchemasObject};

pub(super) fn to_shapes(object: SchemasObject) -> Result<Vec<DefinitionShape>> {
    object.into_iter().map(to_shape).collect()
}

#[derive(Clone, Debug)]
pub enum AllOfItemShape {
    Object(Vec<FieldShape>),
    Ref(ReferenceObject),
}

#[derive(Clone, Debug)]
pub struct FieldShape {
    pub name: ComponentName,
    pub type_shape: TypeShape,
}

#[derive(Clone, Debug)]
pub struct TypeHeaderShape {
    pub name: ComponentName,
    pub doc_comments: DocComments,
    pub is_nullable: bool,
}

#[derive(Clone, Debug)]
pub enum TypeShape {
    Fixed {
        data_type: DataType,
        is_required: bool,
        is_nullable: bool,
    },
    Vec {
        type_shape: Box<TypeShape>,
        is_required: bool,
        is_nullable: bool,
    },
    Ref {
        object: ReferenceObject,
        is_required: bool,
    },
}

impl TypeShape {
    pub fn is_required(&self) -> bool {
        match self {
            TypeShape::Fixed { is_required, .. } => *is_required,
            TypeShape::Vec { is_required, .. } => *is_required,
            TypeShape::Ref { is_required, .. } => *is_required,
        }
    }
}
