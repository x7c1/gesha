mod inline_all_of;
pub use inline_all_of::InlineAllOfShape;

mod inline_enum;
pub use inline_enum::InlineEnumShape;

mod inline_one_of;
pub use inline_one_of::InlineOneOfShape;

mod inline_struct;
pub use inline_struct::InlineStructShape;

use crate::v3_0::components::schemas::type_header_shape::CanConstructHeader;
use gesha_core::conversions::Result;
use openapi_types::v3_0::SchemaObject;

#[derive(Clone, Debug)]
pub enum InlineShape {
    Struct(InlineStructShape),
    AllOf(InlineAllOfShape),
    Enum(InlineEnumShape),
    OneOf(InlineOneOfShape),
}

impl InlineShape {
    pub fn new(_object: SchemaObject) -> Result<Self> {
        todo!()
    }
}

impl CanConstructHeader for InlineShape {
    fn title(&self) -> Option<&str> {
        todo!()
    }

    fn description(&self) -> Option<&str> {
        todo!()
    }

    fn nullable(&self) -> Option<bool> {
        todo!()
    }
}
