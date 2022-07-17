mod components_shapes;
use components_shapes::ComponentsShapes;

mod definition_shape;
use definition_shape::DefinitionShape;

mod post_process;
use post_process::PostProcessor;

mod to_shape;
use to_shape::to_shape;

use crate::conversions::{Result, ToRustType};
use crate::targets::rust_type::{DataType, Modules, StructFieldName};
use openapi_types::v3_0::{ComponentsObject, Document, ReferenceObject, SchemasObject};

impl ToRustType<Document> for Modules {
    fn apply(this: Document) -> Result<Self> {
        let module = this
            .components
            .map(ToRustType::apply)
            .unwrap_or_else(|| Ok(Modules::empty()))?;

        Ok(module)
    }
}

impl ToRustType<ComponentsObject> for Modules {
    fn apply(this: ComponentsObject) -> Result<Self> {
        let to_shapes = |object: SchemasObject| {
            object
                .into_iter()
                .map(to_shape)
                .collect::<Result<Vec<DefinitionShape>>>()
        };
        let mut shapes = ComponentsShapes {
            schemas: this.schemas.map(to_shapes).unwrap_or_else(|| Ok(vec![]))?,
        };
        let defs = PostProcessor::run2(&mut shapes)?;
        shapes.into_modules(defs)
    }
}

#[derive(Clone, Debug)]
enum AllOfItemShape {
    Object(Vec<FieldShape>),
    Ref(ReferenceObject),
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

#[derive(Clone, Debug)]
struct FieldShape {
    name: StructFieldName,
    type_shape: TypeShape,
}

pub fn contains_patch(x: &DataType) -> bool {
    match x {
        DataType::Bool => false,
        DataType::Int32 => false,
        DataType::Int64 => false,
        DataType::Float32 => false,
        DataType::Float64 => false,
        DataType::Option(x) => contains_patch(x),
        DataType::Patch(_) => true,
        DataType::String => false,
        DataType::Vec(x) => contains_patch(x),
        DataType::Custom(_) => false,
    }
}

pub fn is_patch(x: &DataType) -> bool {
    matches!(x, DataType::Patch(_))
}
