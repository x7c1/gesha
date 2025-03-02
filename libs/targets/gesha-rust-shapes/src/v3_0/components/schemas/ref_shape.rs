use crate::v3_0::components::schemas::TypeShape;
use gesha_core::conversions::Result;
use heck::ToUpperCamelCase;

use openapi_types::v3_0::{ReferenceObject, SchemaObject};
type Ref = ReferenceObject<SchemaObject>;

#[derive(Clone, Debug)]
pub struct RefShape {
    pub original: Ref,
    pub is_required: bool,
    pub type_name: String,
}

impl RefShape {
    pub fn new(original: Ref, is_required: bool) -> Result<Self> {
        // TODO: return error if original is not supported format
        let type_name = to_pascal_case(&original);
        Ok(Self {
            original,
            is_required,
            type_name,
        })
    }
}

fn to_pascal_case(target: &Ref) -> String {
    let prefix = "#/components/schemas/";

    let type_name = match target.as_ref() {
        x if x.starts_with(prefix) => x.replace(prefix, ""),
        x => unimplemented!("not implemented: {x}"),
    };
    type_name.to_upper_camel_case()
}

impl From<RefShape> for TypeShape {
    fn from(this: RefShape) -> Self {
        Self::Ref(this)
    }
}
