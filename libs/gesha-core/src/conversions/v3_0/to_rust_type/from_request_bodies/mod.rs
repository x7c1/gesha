mod media_type_shape;
use media_type_shape::MediaTypeShape;

use crate::conversions::Result;
use crate::targets::rust_type::DocComments;
use openapi_types::v3_0::{
    ComponentName, RequestBodiesObject, RequestBodyCase, RequestBodyObject, SchemaCase,
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
pub struct DefinitionShape {
    pub name: ComponentName,
    pub doc_comments: DocComments,
    pub is_required: bool,
    pub contents: Vec<ContentShape>,
}

#[derive(Clone, Debug)]
pub struct ContentShape {
    pub media_type: MediaTypeShape,
    pub schema: SchemaCase,
}

#[derive(Debug)]
struct Shaper {
    name: ComponentName,
    object: RequestBodyObject,
}

impl Shaper {
    fn run(self) -> Result<DefinitionShape> {
        let contents = self
            .object
            .content
            .into_iter()
            .map(|(key, value)| ContentShape {
                media_type: MediaTypeShape::new(key),
                schema: value.schema,
            })
            .collect();

        Ok(DefinitionShape {
            name: self.name,
            doc_comments: DocComments::wrap(self.object.description),
            is_required: self.object.required,
            contents,
        })
    }
}
