mod components_shapes;
use components_shapes::ComponentsShapes;

mod definition_shape;
use definition_shape::DefinitionShape;

mod post_process;
use post_process::PostProcessor;

mod to_shape;
use to_shape::to_shape;

use crate::conversions::{Result, ToRustType};
use crate::targets::rust_type::{DataType, Modules, StructField, StructFieldName, TypeHeader};
use openapi_types::v3_0::{ComponentsObject, Document, ReferenceObject, SchemasObject};
use DefinitionShape::InProcess;

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
        PostProcessor::run(&mut shapes)?;
        shapes.into_modules()
    }
}

#[derive(Clone, Debug)]
enum PostProcess {
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
}

impl From<PostProcess> for DefinitionShape {
    fn from(this: PostProcess) -> Self {
        InProcess(this)
    }
}

#[derive(Clone, Debug)]
enum AllOfItemShape {
    Object(Vec<FieldShape>),
    Ref(ReferenceObject),
}

#[derive(Clone, Debug)]
pub enum TypeShape {
    Fixed(DataType),
    Maybe(Box<TypeShape>),
    Vec(Box<TypeShape>),
    Ref(ReferenceObject),
}

#[derive(Clone, Debug)]
enum FieldShape {
    Fixed(StructField),
    InProcess {
        name: StructFieldName,
        type_shape: TypeShape,
    },
}
