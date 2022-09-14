use crate::conversions::Result;
use crate::targets::rust_type::DocComments;
use openapi_types::v3_0::{
    ComponentName, MediaTypeKey, RequestBodiesObject, RequestBodyCase, RequestBodyObject,
    SchemaCase,
};

pub(super) fn to_shapes(object: RequestBodiesObject) -> Result<Vec<DefinitionShape>> {
    object.into_iter().map(to_shape).collect()
}

fn to_shape(kv: (ComponentName, RequestBodyCase)) -> Result<DefinitionShape> {
    let (field_name, request_body_case) = kv;
    match request_body_case {
        RequestBodyCase::RequestBody(obj) => {
            let (name, object) = (field_name, *obj);
            Shaper { name, object }.run()
        }
        RequestBodyCase::Reference(_) => todo!(),
    }
}

#[derive(Clone, Debug)]
pub(super) struct DefinitionShape {
    name: ComponentName,
    doc_comments: DocComments,
    is_required: bool,
    contents: Vec<ContentShape>,
}

#[derive(Clone, Debug)]
pub(super) enum MediaTypeShape {
    ApplicationXml,
    ApplicationJson,
    Unsupported(String),
}

#[derive(Clone, Debug)]
pub(super) struct ContentShape {
    media_type: MediaTypeShape,
    schema: SchemaCase,
}

#[derive(Debug)]
struct Shaper {
    name: ComponentName,
    object: RequestBodyObject,
}

impl Shaper {
    fn run(self) -> Result<DefinitionShape> {
        println!("{:#?}", self);

        let contents = self
            .object
            .content
            .into_iter()
            .map(|(key, value)| ContentShape {
                media_type: to_media_type(key),
                schema: value.schema,
            })
            .collect();

        Ok(DefinitionShape {
            name: self.name,
            doc_comments: DocComments::new(self.object.description),
            is_required: self.object.required,
            contents,
        })
    }
}

fn to_media_type(key: MediaTypeKey) -> MediaTypeShape {
    // TODO:
    MediaTypeShape::ApplicationJson
}
