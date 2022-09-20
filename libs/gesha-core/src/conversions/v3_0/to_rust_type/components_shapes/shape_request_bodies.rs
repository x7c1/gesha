use crate::conversions::v3_0::to_rust_type::components_shapes::{create_module, ComponentsShapes};
use crate::conversions::v3_0::to_rust_type::from_request_bodies::{
    ContentShape, DefinitionShape, MediaTypeShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::{
    DataType, Definition, EnumVariant, EnumVariantName, Module, RequestBodyDef, TypeHeader,
};
use openapi_types::v3_0::SchemaCase;

impl ComponentsShapes {
    pub fn shape_request_bodies(&mut self) -> Result<Option<Module>> {
        println!("{:#?}", self.request_bodies);

        let definitions = self
            .request_bodies
            .clone()
            .into_iter()
            .map(|shape| self.define_request_body(shape))
            .collect::<Result<Vec<Definition>>>()?;

        create_module("request_bodies", definitions)
    }

    fn define_request_body(&self, shape: DefinitionShape) -> Result<Definition> {
        let header = TypeHeader::new(shape.name.to_string(), shape.doc_comments);
        let variants = shape
            .contents
            .into_iter()
            .filter_map(|x| self.content_shape_to_variant(&x).transpose())
            .collect::<Result<Vec<EnumVariant>>>()?;

        let def = RequestBodyDef::new(header, variants);
        Ok(def.into())
    }

    fn content_shape_to_variant(&self, shape: &ContentShape) -> Result<Option<EnumVariant>> {
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
                Ok(Some(variant))
            }
        }
    }

    fn require_schema_type_name(&self, schema: &SchemaCase) -> Result<String> {
        match schema {
            SchemaCase::Schema(_) => {
                unimplemented!("inline object not supported yet")
            }
            SchemaCase::Reference(x) => self
                .find_schema_definition(x)
                .map(|x| x.type_name().to_string()),
        }
    }
}
