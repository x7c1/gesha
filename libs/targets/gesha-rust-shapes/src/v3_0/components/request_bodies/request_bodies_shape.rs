use crate::v3_0::components::request_bodies::{
    ContentShape, DefinitionShape, MediaTypeShape, ModShape,
};
use gesha_collections::partial_result::MergeOps;
use gesha_collections::seq::TryMapOps;
use gesha_core::conversions::{Output, Result};
use gesha_rust_types::{DocComments, EnumVariantName, MediaTypeDef, ModDef, TypeIdentifier};
use indexmap::IndexMap;
use openapi_types::v3_0::{ComponentName, RequestBodiesObject, RequestBodyCase, RequestBodyObject};
use std::ops::Not;

#[derive(Debug, Clone)]
pub struct RequestBodiesShape {
    pub root: ModShape,
}

impl RequestBodiesShape {
    pub fn shape(maybe: Option<RequestBodiesObject>) -> Output<Self> {
        let (defs, errors) = if let Some(object) = maybe {
            object.try_map(new).merge().into_tuple()
        } else {
            Default::default()
        };
        let this = Self {
            root: ModShape::new(ComponentName::new("request_bodies"), defs),
        };
        Output::new(this, errors)
    }

    pub fn define(self) -> Result<Option<ModDef>> {
        let def = self.root.define()?;
        let maybe = def.defs.is_empty().not().then_some(def);
        Ok(maybe)
    }

    pub fn define_media_type(&self) -> Result<Option<MediaTypeDef>> {
        let translator = self
            .root
            .defs
            .iter()
            .flat_map(|def| def.media_types())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .collect::<IndexMap<EnumVariantName, String>>();

        let def = translator
            .is_empty()
            .not()
            .then_some(MediaTypeDef { translator });

        Ok(def)
    }
}

fn new(kv: (ComponentName, RequestBodyCase)) -> Result<DefinitionShape> {
    let (field_name, request_body_case) = kv;
    match request_body_case {
        RequestBodyCase::RequestBody(object) => shape(field_name, *object),
        RequestBodyCase::Reference(_) => unimplemented!(),
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
        name: TypeIdentifier::parse(name)?,
        doc_comments: DocComments::wrap(object.description),
        is_required: object.required,
        contents,
    })
}
