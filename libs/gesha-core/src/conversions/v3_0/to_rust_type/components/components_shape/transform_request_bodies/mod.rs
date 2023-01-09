use crate::conversions::v3_0::to_rust_type::components::request_bodies::{
    ContentShape, DefinitionShape, MediaTypeShape,
};
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Result;
use crate::targets::rust_type::{DataType, EnumVariant, EnumVariantName, MediaTypeVariant};
use openapi_types::v3_0::SchemaCase;

pub fn transform_request_bodies(mut shape: ComponentsShape) -> Result<ComponentsShape> {
    let transformer = Transformer {
        snapshot: shape.clone(),
    };
    let defs = shape.request_bodies.root.defs;
    let request_bodies = defs
        .into_iter()
        .map(|x| transformer.run(x))
        .collect::<Result<Vec<_>>>()?;

    shape.request_bodies.root.defs = request_bodies;
    Ok(shape)
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
                // already processed
                Ok(shape)
            }
        }
    }

    fn require_schema_type_name(&self, schema: &SchemaCase) -> Result<String> {
        match schema {
            SchemaCase::Reference(x) => self
                .snapshot
                .find_type_definition(x)
                .map(|x| x.type_name().to_string()),
            SchemaCase::Schema(_) => {
                unimplemented!("inline object not supported yet")
            }
        }
    }
}
