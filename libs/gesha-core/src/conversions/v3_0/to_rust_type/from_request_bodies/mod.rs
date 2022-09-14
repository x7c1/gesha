use crate::conversions::Result;
use openapi_types::v3_0::{ComponentName, RequestBodiesObject, RequestBodyCase, RequestBodyObject};

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
pub(super) enum DefinitionShape {
    Enum,
}

#[derive(Debug)]
struct Shaper {
    name: ComponentName,
    object: RequestBodyObject,
}

impl Shaper {
    fn run(self) -> Result<DefinitionShape> {
        println!("{:#?}", self);
        panic!();
        Ok(DefinitionShape::Enum)
    }
}
