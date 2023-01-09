mod media_type_shape;
pub use media_type_shape::MediaTypeShape;

mod mod_shape;
pub use mod_shape::ModShape;

mod request_bodies_shape;
pub use request_bodies_shape::RequestBodiesShape;

use crate::conversions::Result;
use crate::targets::rust_type::{
    Definition, DocComments, EnumVariantName, MediaTypeVariant, RequestBodyDef, TypeHeader,
};
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

    pub fn define(self) -> Result<Definition> {
        let header = TypeHeader::new(self.name.to_string(), self.doc_comments);
        let variants = self
            .contents
            .into_iter()
            .filter_map(|x| content_shape_to_variant(x).transpose())
            .collect::<Result<Vec<MediaTypeVariant>>>()?;

        let def = RequestBodyDef::new(header, variants);
        Ok(def.into())
    }
}

impl TryFrom<DefinitionShape> for Definition {
    type Error = crate::conversions::Error;

    fn try_from(this: DefinitionShape) -> Result<Self> {
        this.define()
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

fn content_shape_to_variant(shape: ContentShape) -> Result<Option<MediaTypeVariant>> {
    match shape {
        ContentShape::Defined(x) => Ok(x),
        ContentShape::Raw { .. } => {
            // todo: return error
            unimplemented!()
        }
    }
}
