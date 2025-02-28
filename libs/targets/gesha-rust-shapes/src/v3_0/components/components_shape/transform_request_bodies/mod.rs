use crate::misc::MapOutput;
use crate::v3_0::components::request_bodies::{ContentShape, DefinitionShape, MediaTypeShape};
use crate::v3_0::components::ComponentsShape;
use gesha_core::conversions::Error::ReferenceObjectNotFound;
use gesha_core::conversions::Result;
use gesha_rust_types::{DataType, EnumVariant, EnumVariantName, MediaTypeVariant};
use openapi_types::v3_0::SchemaCase;

pub fn transform_request_bodies(mut shape: ComponentsShape) -> Result<ComponentsShape> {
    let transformer = Transformer {
        snapshot: shape.clone(),
    };
    let defs = shape.request_bodies.root.defs;
    let request_bodies = defs.map_output(|x| transformer.run(x)).to_result()?;
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
            .map_output(|x| self.content_shape_to_variant(x))
            .to_result()?;

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
                .schemas
                .find_type_name(x)
                .ok_or_else(|| ReferenceObjectNotFound(x.clone().into()))
                .map(|name| name.clone().into()),

            SchemaCase::Schema(_) => {
                unimplemented!("inline object not supported yet")
            }
        }
    }
}
