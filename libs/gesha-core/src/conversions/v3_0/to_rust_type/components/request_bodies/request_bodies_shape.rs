use crate::conversions::v3_0::to_rust_type::components::request_bodies::{
    ContentShape, DefinitionShape, MediaTypeShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::{DocComments, EnumVariantName, MediaTypeDef};
use indexmap::IndexMap;
use openapi_types::v3_0::{ComponentName, RequestBodiesObject, RequestBodyCase, RequestBodyObject};
use std::ops::Not;

#[derive(Debug, Clone, Default)]
pub struct RequestBodiesShape(Vec<DefinitionShape>);

impl RequestBodiesShape {
    pub fn from(object: RequestBodiesObject) -> Result<Self> {
        let xs = object.into_iter().map(new).collect::<Result<Vec<_>>>()?;
        Ok(RequestBodiesShape(xs))
    }

    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn iter(&self) -> impl Iterator<Item = &DefinitionShape> {
        self.0.iter()
    }

    pub fn define_media_type(&self) -> Result<Option<MediaTypeDef>> {
        let translator = self
            .iter()
            .flat_map(|def| def.media_types())
            .collect::<IndexMap<EnumVariantName, String>>();

        let def = translator
            .is_empty()
            .not()
            .then_some(MediaTypeDef { translator });

        Ok(def)
    }
}

impl FromIterator<DefinitionShape> for RequestBodiesShape {
    fn from_iter<T: IntoIterator<Item = DefinitionShape>>(iter: T) -> Self {
        let xs = iter.into_iter().collect();
        Self(xs)
    }
}

impl IntoIterator for RequestBodiesShape {
    type Item = <Vec<DefinitionShape> as IntoIterator>::Item;
    type IntoIter = <Vec<DefinitionShape> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}

fn new(kv: (ComponentName, RequestBodyCase)) -> Result<DefinitionShape> {
    let (field_name, request_body_case) = kv;
    match request_body_case {
        RequestBodyCase::RequestBody(object) => shape(field_name, *object),
        RequestBodyCase::Reference(_) => todo!(),
    }
}

fn shape(name: ComponentName, object: RequestBodyObject) -> Result<DefinitionShape> {
    let contents = object
        .content
        .into_iter()
        .map(|(key, value)| ContentShape::Raw {
            media_type: MediaTypeShape::new(key),
            schema: value.schema,
        })
        .collect();

    Ok(DefinitionShape {
        name,
        doc_comments: DocComments::wrap(object.description),
        is_required: object.required,
        contents,
    })
}
