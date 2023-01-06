use crate::conversions::v3_0::to_rust_type::components::components_shape::create_module;
use crate::conversions::v3_0::to_rust_type::components::request_bodies::{
    ContentShape, DefinitionShape, MediaTypeShape, RequestBodiesShape,
};
use crate::conversions::v3_0::to_rust_type::components::ComponentsShapes;
use crate::conversions::Result;
use crate::targets::rust_type::{
    DataType, Definition, Definitions, EnumVariant, EnumVariantName, MediaTypeVariant, ModDef,
    RequestBodyDef, TypeHeader,
};
use openapi_types::v3_0::SchemaCase;

pub fn define_request_bodies(shapes: RequestBodiesShape) -> Result<Option<ModDef>> {
    let definitions = shapes
        .into_iter()
        .map(|shape| define(shape))
        .collect::<Result<Definitions>>()?;

    create_module("request_bodies", definitions)
}

fn define(shape: DefinitionShape) -> Result<Definition> {
    let header = TypeHeader::new(shape.name.to_string(), shape.doc_comments);
    let variants = shape
        .contents
        .into_iter()
        .filter_map(|x| content_shape_to_variant(x).transpose())
        .collect::<Result<Vec<MediaTypeVariant>>>()?;

    let def = RequestBodyDef::new(header, variants);
    Ok(def.into())
}

fn content_shape_to_variant(shape: ContentShape) -> Result<Option<MediaTypeVariant>> {
    match shape {
        ContentShape::Raw { .. } => {
            // todo: return error
            unimplemented!()
        }
        ContentShape::Defined(x) => Ok(x),
    }
}
