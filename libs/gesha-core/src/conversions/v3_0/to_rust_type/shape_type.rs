use crate::conversions::v3_0::to_rust_type::shape_schema_object_type;
use crate::conversions::v3_0::to_rust_type::TypeShape;
use crate::conversions::Result;
use openapi_types::v3_0::SchemaCase::{Reference, Schema};
use openapi_types::v3_0::{ReferenceObject, SchemaCase};

pub(super) fn shape_type(schema_case: SchemaCase) -> Result<TypeShape> {
    match schema_case {
        Schema(object) => shape_schema_object_type(*object),
        Reference(object) => shape_schema_reference_type(object),
    }
}

fn shape_schema_reference_type(object: ReferenceObject) -> Result<TypeShape> {
    Ok(TypeShape::Ref(object))
}
