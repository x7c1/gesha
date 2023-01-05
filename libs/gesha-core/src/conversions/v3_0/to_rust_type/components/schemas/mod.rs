mod all_of_item_shape;
pub use all_of_item_shape::AllOfItemShape;

mod all_of_shape;
pub use all_of_shape::AllOfShape;

mod definition_shape;
pub use definition_shape::{DefinitionShape, TypeDefinitionShape};

mod field_shape;
pub use field_shape::FieldShape;

mod mod_shape;
pub use mod_shape::ModShape;

mod post_processor;
pub use post_processor::PostProcessor;

mod struct_shape;
pub use struct_shape::StructShape;

mod to_shape;
use to_shape::to_shape;

mod to_type_shape;
use to_type_shape::to_type_shape;

mod type_header_shape;
pub use type_header_shape::TypeHeaderShape;

mod type_path;
pub use type_path::TypePath;

mod to_field_shapes;

use crate::conversions::Result;
use crate::targets::rust_type::DataType;
use openapi_types::v3_0::{ReferenceObject, SchemaObject, SchemasObject};

pub fn to_schemas_shape(object: SchemasObject) -> Result<Vec<DefinitionShape>> {
    object.into_iter().map(to_shape).collect()
}

#[derive(Clone, Debug)]
pub enum TypeShape {
    Fixed {
        data_type: DataType,
        is_required: bool,
        is_nullable: bool,
    },
    Array {
        type_shape: Box<TypeShape>,
        is_required: bool,
        is_nullable: bool,
    },
    Ref {
        object: ReferenceObject<SchemaObject>,
        is_required: bool,
    },
    Expanded {
        type_path: TypePath,
        is_required: bool,
        is_nullable: bool,
    },
    InlineObject {
        object: SchemaObject,
        is_required: bool,
        is_nullable: bool,
    },
    Higher {
        type_shape: Box<TypeShape>,
        type_name: String,
    },
}

impl TypeShape {
    pub fn is_required(&self) -> bool {
        match self {
            TypeShape::Fixed { is_required, .. } => *is_required,
            TypeShape::Array { is_required, .. } => *is_required,
            TypeShape::Ref { is_required, .. } => *is_required,
            TypeShape::InlineObject { is_required, .. } => *is_required,
            TypeShape::Expanded { is_required, .. } => *is_required,
            TypeShape::Higher { .. } => false,
        }
    }
}
