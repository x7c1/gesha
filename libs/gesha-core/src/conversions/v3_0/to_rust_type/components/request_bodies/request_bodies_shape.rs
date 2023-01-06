use crate::conversions::v3_0::to_rust_type::components::request_bodies::{
    ContentShape, DefinitionShape, MediaTypeShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::DocComments;
use openapi_types::v3_0::{
    ComponentName, RequestBodiesObject, RequestBodyCase, RequestBodyObject, SchemaCase,
};

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
        RequestBodyCase::RequestBody(obj) => {
            let (name, object) = (field_name, *obj);
            Shaper { name, object }.run()
        }
        RequestBodyCase::Reference(_) => todo!(),
    }
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
            .map(|(key, value)| ContentShape::Raw {
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
