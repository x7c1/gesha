mod media_type_shape;
pub use media_type_shape::MediaTypeShape;

mod request_bodies_shape;
pub use request_bodies_shape::RequestBodiesShape;

use crate::conversions::Result;
use crate::targets::rust_type::{DocComments, EnumVariantName, MediaTypeVariant};
use openapi_types::v3_0::{
    ComponentName, RequestBodiesObject, RequestBodyCase, RequestBodyObject, SchemaCase,
};

#[derive(Clone, Debug)]
pub struct DefinitionShape {
    pub name: ComponentName,
    pub doc_comments: DocComments,
    pub is_required: bool,
    pub contents: Vec<ContentShape>,
}

// impl DefinitionShape {
//     pub fn translate_media_types(
//         &self,
//     ) -> impl Iterator<Item = (EnumVariantName, &'static str)> + '_ {
//         self.contents
//             .iter()
//             .flat_map(|content| match content.media_type {
//                 MediaTypeShape::ApplicationJson => {
//                     Some((EnumVariantName::new("ApplicationJson"), "application/json"))
//                 }
//                 MediaTypeShape::Unsupported(_) => None,
//             })
//     }
// }

#[derive(Clone, Debug)]
pub enum ContentShape {
    Raw {
        media_type: MediaTypeShape,
        schema: SchemaCase,
    },
    Defined(Option<MediaTypeVariant>),
}
