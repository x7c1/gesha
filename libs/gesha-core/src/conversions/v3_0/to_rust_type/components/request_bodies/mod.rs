mod media_type_shape;
pub use media_type_shape::MediaTypeShape;

mod request_bodies_shape;
pub use request_bodies_shape::RequestBodiesShape;

use crate::targets::rust_type::{DocComments, EnumVariantName, MediaTypeVariant};
use openapi_types::v3_0::{ComponentName, SchemaCase};

#[derive(Clone, Debug)]
pub struct DefinitionShape {
    pub name: ComponentName,
    pub doc_comments: DocComments,
    pub is_required: bool,
    pub contents: Vec<ContentShape>,
}

impl DefinitionShape {
    pub fn media_types(&self) -> impl Iterator<Item = (EnumVariantName, String)> + '_ {
        use ContentShape::{Defined, Raw};
        self.contents.iter().flat_map(|content| match content {
            Defined(None) => None,
            Defined(Some(x)) => Some((x.variant.name.clone(), x.header_value.clone())),
            Raw { .. } => unimplemented!("return error"),
        })
    }
}

#[derive(Clone, Debug)]
pub enum ContentShape {
    Raw {
        media_type: MediaTypeShape,
        schema: SchemaCase,
    },
    Defined(Option<MediaTypeVariant>),
}
