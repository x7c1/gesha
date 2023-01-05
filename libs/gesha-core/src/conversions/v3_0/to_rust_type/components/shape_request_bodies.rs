use crate::conversions::v3_0::to_rust_type::components::request_bodies::{
    ContentShape, DefinitionShape, MediaTypeShape,
};
use crate::conversions::v3_0::to_rust_type::components::{create_module, ComponentsShapes};
use crate::conversions::Result;
use crate::targets::rust_type::{
    DataType, Definition, Definitions, EnumVariant, EnumVariantName, MediaTypeVariant, ModDef,
    RequestBodyDef, TypeHeader,
};
use openapi_types::v3_0::SchemaCase;

impl ComponentsShapes {
    pub fn shape_request_bodies(&self) -> Result<Option<ModDef>> {
        let definitions = self
            .request_bodies
            .clone()
            .into_iter()
            .map(|shape| self.define_request_body(shape))
            .collect::<Result<Definitions>>()?;

        create_module("request_bodies", definitions)
    }

    fn define_request_body(&self, shape: DefinitionShape) -> Result<Definition> {
        let header = TypeHeader::new(shape.name.to_string(), shape.doc_comments);
        let variants = shape
            .contents
            .into_iter()
            .filter_map(|x| self.content_shape_to_variant(&x).transpose())
            .collect::<Result<Vec<MediaTypeVariant>>>()?;

        let def = RequestBodyDef::new(header, variants);
        Ok(def.into())
    }

    fn content_shape_to_variant(&self, shape: &ContentShape) -> Result<Option<MediaTypeVariant>> {
        match &shape.media_type {
            MediaTypeShape::Unsupported(_) => {
                // ignore unsupported media type
                Ok(None)
            }
            media_type_shape => {
                let type_name = self.require_schema_type_name(&shape.schema)?;
                let variant = EnumVariant::tuple(
                    EnumVariantName::new(media_type_shape),
                    vec![DataType::Custom(format!("super::schemas::{}", type_name))],
                    vec![],
                );
                Ok(Some(MediaTypeVariant {
                    header_value: media_type_shape.as_ref().to_string(),
                    variant,
                }))
            }
        }
    }

    fn require_schema_type_name(&self, schema: &SchemaCase) -> Result<String> {
        match schema {
            SchemaCase::Schema(_) => {
                unimplemented!("inline object not supported yet")
            }
            SchemaCase::Reference(x) => self
                .find_type_definition(x)
                .map(|x| x.type_name().to_string()),
        }
    }
}
