use crate::conversions::v3_0::to_rust_type::from_schemas::DefinitionShape;
use crate::conversions::Result;
use openapi_types::v3_0::{ComponentName, RequestBodyCase, RequestBodyObject};

pub(super) fn to_shape(kv: (ComponentName, RequestBodyCase)) -> Result<DefinitionShape> {
    let (field_name, request_body_case) = kv;
    match request_body_case {
        RequestBodyCase::RequestBody(obj) => {
            let (name, object) = (field_name, *obj);
            Shaper { name, object }.run()
        }
        RequestBodyCase::Reference(_) => todo!(),
    }
}

struct Shaper {
    name: ComponentName,
    object: RequestBodyObject,
}

impl Shaper {
    fn run(self) -> Result<DefinitionShape> {
        unimplemented!()
    }
}
