mod inline_schema;
pub use inline_schema::InlineSchema;

use crate::v3_0::components::schemas::type_header_shape::HeaderBody;
use crate::v3_0::components::schemas::{Optionality, TypeShape};
use gesha_core::conversions::Result;
use openapi_types::v3_0::SchemaObject;

#[derive(Clone, Debug)]
pub enum InlineShape {
    Struct(InlineSchema),
    Enum(InlineSchema),
    AllOf(InlineSchema),
    OneOf(InlineSchema),
}

impl InlineShape {
    pub fn new(object: SchemaObject, optionality: Optionality) -> Result<Self> {
        let has_all_of = object.all_of.is_some();
        if has_all_of {
            return InlineSchema::new(object, optionality).map(Self::AllOf);
        }

        let has_one_of = object.one_of.is_some();
        if has_one_of {
            return InlineSchema::new(object, optionality).map(Self::OneOf);
        }

        let has_enum = object.enum_values.is_some();
        if has_enum {
            return InlineSchema::new(object, optionality).map(Self::Enum);
        }

        InlineSchema::new(object, optionality).map(Self::Struct)
    }

    pub fn get_optionality(&self) -> &Optionality {
        &self.get_inline_object().optionality
    }

    pub fn set_optionality(&mut self, optionality: Optionality) {
        let inline = self.get_mut_inline_object();
        inline.optionality = optionality;
    }

    pub fn set_required(&mut self, required: bool) {
        let inline = self.get_mut_inline_object();
        inline.optionality.is_required = required;
    }

    fn get_inline_object(&self) -> &InlineSchema {
        match self {
            InlineShape::Struct(inline) => inline,
            InlineShape::Enum(inline) => inline,
            InlineShape::AllOf(inline) => inline,
            InlineShape::OneOf(inline) => inline,
        }
    }

    fn get_mut_inline_object(&mut self) -> &mut InlineSchema {
        match self {
            InlineShape::Struct(inline) => inline,
            InlineShape::Enum(inline) => inline,
            InlineShape::AllOf(inline) => inline,
            InlineShape::OneOf(inline) => inline,
        }
    }
}

impl From<&InlineShape> for HeaderBody {
    fn from(value: &InlineShape) -> Self {
        value.get_inline_object().generate_header_body()
    }
}

impl From<InlineShape> for TypeShape {
    fn from(value: InlineShape) -> Self {
        Self::Inline(Box::new(value))
    }
}
