use crate::conversions::v3_0::to_rust_type::{shape_type, FieldShape, TypeShape};
use crate::conversions::Result;
use crate::targets::rust_type::{DataType, StructField, StructFieldName};
use openapi_types::v3_0::{
    RequiredSchemaFields, SchemaCase, SchemaFieldName, SchemaObject, SchemaProperties,
};

pub(super) fn object_to_field_shapes(object: SchemaObject) -> Result<Vec<FieldShape>> {
    object
        .properties
        .map(ToFieldShapes::by(object.required))
        .unwrap_or(Ok(vec![]))
}

/// SchemaProperties -> Vec<FieldShape>
struct ToFieldShapes {
    required: Option<RequiredSchemaFields>,
}

impl ToFieldShapes {
    fn by(
        required: Option<RequiredSchemaFields>,
    ) -> impl FnOnce(SchemaProperties) -> Result<Vec<FieldShape>> {
        |props| ToFieldShapes { required }.apply(props)
    }

    fn apply(self, props: SchemaProperties) -> Result<Vec<FieldShape>> {
        props
            .into_iter()
            .map(|(name, case)| self.to_field(name, case))
            .collect()
    }

    fn to_field(&self, name: SchemaFieldName, case: SchemaCase) -> Result<FieldShape> {
        match shape_type(case)? {
            TypeShape::Fixed(data_type) => {
                let field = self.new_struct_field(name, data_type);
                Ok(FieldShape::Fixed(field))
            }
            type_shape => Ok(FieldShape::InProcess {
                is_optional: !self.is_required(&name),
                name: StructFieldName::new(name),
                type_shape,
            }),
        }
    }

    fn is_required(&self, name: &SchemaFieldName) -> bool {
        match &self.required {
            Some(required) => required.contains(name.as_ref()),
            None => false,
        }
    }

    fn new_struct_field(&self, name: SchemaFieldName, mut data_type: DataType) -> StructField {
        if !self.is_required(&name) {
            data_type = DataType::Option(Box::new(data_type));
        }
        StructField {
            name: StructFieldName::new(name),
            data_type,
        }
    }
}
