use crate::conversions::v3_0::to_rust_type::components::components_shape::create_module;
use crate::conversions::v3_0::to_rust_type::components::request_bodies::{
    ContentShape, DefinitionShape, RequestBodiesShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::{
    Definition, Definitions, MediaTypeVariant, ModDef, RequestBodyDef, TypeHeader,
};

pub fn define_request_bodies(shape: RequestBodiesShape) -> Result<Option<ModDef>> {
    let definitions = shape
        .into_iter()
        .map(define)
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
