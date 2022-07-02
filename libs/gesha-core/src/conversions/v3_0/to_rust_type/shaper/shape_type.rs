use crate::conversions::v3_0::to_rust_type::shaper::shape_schema_object_type::shape_schema_object_type;
use crate::conversions::v3_0::to_rust_type::TypeShape;
use crate::conversions::Result;
use openapi_types::v3_0::SchemaCase;
use openapi_types::v3_0::SchemaCase::{Reference, Schema};

pub(super) fn shape_type(schema_case: SchemaCase) -> Result<TypeShape> {
    match schema_case {
        Schema(object) => shape_schema_object_type(*object),
        Reference(object) => Ok(TypeShape::Ref(object)),
    }
}
