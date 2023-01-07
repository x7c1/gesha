use crate::conversions::v3_0::to_rust_type::components::components_shape::create_module;
use crate::conversions::v3_0::to_rust_type::components::request_bodies::{
    ContentShape, DefinitionShape, MediaTypeShape, RequestBodiesShape,
};
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Result;
use crate::targets::rust_type::{
    DataType, Definition, Definitions, EnumVariant, EnumVariantName, MediaTypeVariant, ModDef,
    RequestBodyDef, TypeHeader,
};
use openapi_types::v3_0::SchemaCase;

pub fn transform_request_bodies(mut shapes: ComponentsShape) -> Result<ComponentsShape> {
    let transformer = Transformer {
        snapshot: shapes.clone(),
    };
    let request_bodies = shapes
        .request_bodies
        .into_iter()
        .map(|x| transformer.run(x))
        .collect::<Result<RequestBodiesShape>>()?;

    shapes.request_bodies = request_bodies;
    Ok(shapes)
}

struct Transformer {
    snapshot: ComponentsShape,
}

impl Transformer {
    fn run(&self, mut shape: DefinitionShape) -> Result<DefinitionShape> {
        let defined = shape
            .contents
            .into_iter()
            .map(|x| self.content_shape_to_variant(x))
            .collect::<Result<Vec<ContentShape>>>()?;

        shape.contents = defined;
        Ok(shape)
    }

    fn content_shape_to_variant(&self, shape: ContentShape) -> Result<ContentShape> {
        match shape {
            ContentShape::Raw {
                media_type: MediaTypeShape::Unsupported(_),
                ..
            } => {
                // ignore unsupported media type
                Ok(ContentShape::Defined(None))
            }
            ContentShape::Raw { media_type, schema } => {
                let type_name = self.require_schema_type_name(&schema)?;
                let variant = EnumVariant::tuple(
                    EnumVariantName::new(&media_type),
                    vec![DataType::Custom(format!("super::schemas::{}", type_name))],
                    vec![],
                );
                Ok(ContentShape::Defined(Some(MediaTypeVariant {
                    header_value: media_type.as_ref().to_string(),
                    variant,
                })))
            }
            ContentShape::Defined(_) => {
                unimplemented!()
            }
        }
    }

    fn require_schema_type_name(&self, schema: &SchemaCase) -> Result<String> {
        match schema {
            SchemaCase::Schema(_) => {
                unimplemented!("inline object not supported yet")
            }
            SchemaCase::Reference(x) => self
                .snapshot
                .find_type_definition(x)
                .map(|x| x.type_name().to_string()),
        }
    }
}
