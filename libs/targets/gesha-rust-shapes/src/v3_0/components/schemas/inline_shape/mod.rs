mod inline_all_of;
pub use inline_all_of::InlineAllOfShape;

mod inline_enum;
pub use inline_enum::InlineEnumShape;

mod inline_one_of;
pub use inline_one_of::InlineOneOfShape;

mod inline_struct;
pub use inline_struct::InlineStructShape;

mod inline_schema_shape;
pub use inline_schema_shape::InlineSchemaShape;

use crate::v3_0::components::schemas::type_header_shape::{HeaderParts, HeaderPartsGenerator};
use crate::v3_0::components::schemas::Optionality;
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
    pub fn new(object: SchemaObject, optionality: Optionality) -> Result<Self> {
        let has_all_of = object
            .all_of
            .as_ref()
            .map(|x| !x.is_empty())
            .unwrap_or(false);

        if has_all_of {
            return InlineAllOfShape::new(object, optionality).map(Self::AllOf);
        }

        let has_one_of = object
            .one_of
            .as_ref()
            .map(|x| !x.is_empty())
            .unwrap_or(false);

        if has_one_of {
            return InlineOneOfShape::new(object, optionality).map(Self::OneOf);
        }

        let has_enum = object
            .enum_values
            .as_ref()
            .map(|x| !x.is_empty())
            .unwrap_or(false);

        if has_enum {
            return InlineEnumShape::new(object, optionality).map(Self::Enum);
        }

        InlineStructShape::new(object, optionality).map(Self::Struct)
    }

    pub fn optionality(&self) -> &Optionality {
        match self {
            InlineShape::Struct(InlineStructShape { optionality, .. }) => optionality,
            InlineShape::Enum(InlineEnumShape { optionality, .. }) => optionality,
            InlineShape::AllOf(InlineAllOfShape { optionality, .. }) => optionality,
            InlineShape::OneOf(InlineOneOfShape { optionality, .. }) => optionality,
        }
    }

    pub fn set_optionality(&mut self, x: Optionality) {
        match self {
            InlineShape::Struct(InlineStructShape {
                ref mut optionality,
                ..
            }) => *optionality = x,

            InlineShape::Enum(InlineEnumShape {
                ref mut optionality,
                ..
            }) => *optionality = x,

            InlineShape::AllOf(InlineAllOfShape {
                ref mut optionality,
                ..
            }) => *optionality = x,

            InlineShape::OneOf(InlineOneOfShape {
                ref mut optionality,
                ..
            }) => *optionality = x,
        }
    }

    pub fn set_required(&mut self, required: bool) {
        match self {
            InlineShape::Struct(InlineStructShape {
                ref mut optionality,
                ..
            }) => optionality.is_required = required,

            InlineShape::Enum(InlineEnumShape {
                ref mut optionality,
                ..
            }) => optionality.is_required = required,

            InlineShape::AllOf(InlineAllOfShape {
                ref mut optionality,
                ..
            }) => optionality.is_required = required,

            InlineShape::OneOf(InlineOneOfShape {
                ref mut optionality,
                ..
            }) => optionality.is_required = required,
        }
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
