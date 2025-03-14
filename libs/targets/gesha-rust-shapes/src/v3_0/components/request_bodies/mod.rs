mod media_type_shape;
pub use media_type_shape::MediaTypeShape;

mod mod_shape;
pub use mod_shape::ModShape;

mod request_bodies_shape;
pub use request_bodies_shape::RequestBodiesShape;

use gesha_core::broken;
use gesha_core::conversions::Result;
use gesha_rust_types::{
    Definition, DeriveAttribute, DocComments, EnumVariantName, MediaTypeVariant, RequestBodyDef,
    SerdeAttribute, TypeHeader,
};
use openapi_types::v3_0::{ComponentName, SchemaCase};

#[derive(Clone, Debug)]
pub struct DefinitionShape {
    pub name: ComponentName,
    pub doc_comments: Option<DocComments>,
    pub is_required: bool,
    pub contents: Vec<ContentShape>,
}

impl DefinitionShape {
    pub fn media_types(&self) -> impl Iterator<Item = Result<(EnumVariantName, String)>> + '_ {
        use ContentShape::{Defined, Raw};
        self.contents.iter().filter_map(|content| match content {
            Defined(None) => None,
            Defined(Some(x)) => Some(Ok((x.variant.name.clone(), x.header_value.clone()))),
            Raw { .. } => Some(Err(broken!(content))),
        })
    }

    pub fn define(self) -> Result<Definition> {
        let header = TypeHeader::new(
            self.name.to_string(),
            self.doc_comments,
            vec![SerdeAttribute::Untagged],
            DeriveAttribute::all(),
        );
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
    type Error = gesha_core::conversions::Error;

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
        ContentShape::Raw { .. } => Err(broken!(shape)),
    }
}
