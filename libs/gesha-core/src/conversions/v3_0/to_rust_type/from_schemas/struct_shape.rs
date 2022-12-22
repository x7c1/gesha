use crate::conversions::v3_0::to_rust_type::from_schemas::{FieldShape, TypeHeaderShape};

#[derive(Clone, Debug)]
pub struct StructShape {
    pub header: TypeHeaderShape,
    pub shapes: Vec<FieldShape>,
}
