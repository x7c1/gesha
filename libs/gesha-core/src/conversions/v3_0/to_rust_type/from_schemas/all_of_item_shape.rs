use crate::conversions::v3_0::to_rust_type::from_schemas::to_field_shapes::to_field_shapes;
use crate::conversions::v3_0::to_rust_type::from_schemas::FieldShape;
use crate::conversions::Result;
use openapi_types::v3_0::{ReferenceObject, SchemaCase, SchemaObject};

#[derive(Clone, Debug)]
pub enum AllOfItemShape {
    Object(Vec<FieldShape>),
    Ref(ReferenceObject<SchemaObject>),
}

impl AllOfItemShape {
    pub fn from_schema_case(case: SchemaCase) -> Result<Self> {
        let shape = match case {
            SchemaCase::Schema(object) => Self::from_schema_object(*object)?,
            SchemaCase::Reference(x) => Self::Ref(x),
        };
        Ok(shape)
    }
    pub fn from_schema_object(object: SchemaObject) -> Result<Self> {
        let shapes = to_field_shapes(object.properties, object.required)?;
        Ok(Self::Object(shapes))
    }
}
