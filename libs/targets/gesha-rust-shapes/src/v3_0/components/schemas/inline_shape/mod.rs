mod inline_all_of;
pub use inline_all_of::InlineAllOfShape;

mod inline_enum;
pub use inline_enum::InlineEnumShape;

mod inline_one_of;
pub use inline_one_of::InlineOneOfShape;

mod inline_struct;
pub use inline_struct::InlineStructShape;

use crate::v3_0::components::schemas::type_header_shape::{HeaderParts, HeaderPartsGenerator};
use gesha_core::conversions::Result;
use openapi_types::v3_0::SchemaObject;

#[derive(Clone, Debug)]
pub enum InlineShape {
    Struct(InlineStructShape),
    Enum(InlineEnumShape),
    AllOf(InlineAllOfShape),
    OneOf(InlineOneOfShape),
}

impl InlineShape {
    pub fn new(object: SchemaObject) -> Result<Self> {
        let has_all_of = object
            .all_of
            .as_ref()
            .map(|x| !x.is_empty())
            .unwrap_or(false);

        if has_all_of {
            return InlineAllOfShape::new(object).map(Self::AllOf);
        }

        let has_one_of = object
            .one_of
            .as_ref()
            .map(|x| !x.is_empty())
            .unwrap_or(false);

        if has_one_of {
            return InlineOneOfShape::new(object).map(Self::OneOf);
        }

        let has_enum = object
            .enum_values
            .as_ref()
            .map(|x| !x.is_empty())
            .unwrap_or(false);

        if has_enum {
            return InlineEnumShape::new(object).map(Self::Enum);
        }

        InlineStructShape::new(object).map(Self::Struct)
    }
}

impl HeaderPartsGenerator for InlineShape {
    fn generate(&self) -> HeaderParts {
        match self {
            InlineShape::Struct(shape) => shape.generate_header_parts(),
            InlineShape::Enum(shape) => shape.generate_header_parts(),
            InlineShape::AllOf(shape) => shape.generate_header_parts(),
            InlineShape::OneOf(shape) => shape.generate_header_parts(),
        }
    }
}
