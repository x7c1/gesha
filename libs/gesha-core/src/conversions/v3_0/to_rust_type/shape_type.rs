use crate::conversions::v3_0::to_rust_type::shape_schema_object_type;
use crate::conversions::v3_0::to_rust_type::TypeShape;
use crate::conversions::Result;
use crate::targets::rust_type::DataType;
use openapi_types::v3_0::SchemaCase::{Reference, Schema};
use openapi_types::v3_0::{ReferenceObject, SchemaCase};

pub fn shape_type(schema_case: SchemaCase) -> Result<TypeShape> {
    match schema_case {
        Schema(object) => shape_schema_object_type(*object),
        Reference(object) => shape_schema_reference_type(object),
    }
}

fn shape_schema_reference_type(object: ReferenceObject) -> Result<TypeShape> {
    let type_name = match String::from(object) {
        x if x.starts_with("#/components/schemas/") => {
            // TODO: change location to relative paths by TypeShape::Ref
            // if using "#/components/responses/" etc
            x.replace("#/components/schemas/", "")
        }
        x => unimplemented!("not implemented: {x}"),
    };

    Ok(TypeShape::Fixed(DataType::Custom(type_name)))
}
