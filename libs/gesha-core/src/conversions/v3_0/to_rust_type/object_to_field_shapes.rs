use crate::conversions::v3_0::to_rust_type::{shape_type, FieldShape, TypeShape};
use crate::conversions::Result;
use crate::targets::rust_type::{DataType, StructField, StructFieldName};
use openapi_types::v3_0::{
    RequiredSchemaFields, SchemaCase, SchemaFieldName, SchemaObject, SchemaProperties,
};

pub(super) fn object_to_field_shapes(object: SchemaObject) -> Result<Vec<FieldShape>> {
    let to_fields = |properties| {
        let factory = FieldsFactory {
            required: object.required,
        };
        factory.apply(properties)
    };
    object.properties.map(to_fields).unwrap_or(Ok(vec![]))
}

/// SchemaProperties -> Vec<FieldShape>
struct FieldsFactory {
    pub required: Option<RequiredSchemaFields>,
}

impl FieldsFactory {
    pub fn apply(self, props: SchemaProperties) -> Result<Vec<FieldShape>> {
        props
            .into_iter()
            .map(|(name, case)| self.to_field(name, case))
            .collect()
    }

    fn to_field(&self, name: SchemaFieldName, case: SchemaCase) -> Result<FieldShape> {
        match shape_type(case)? {
            TypeShape::Fixed(mut data_type) => {
                if !self.is_required(&name) {
                    data_type = DataType::Option(Box::new(data_type));
                }
                Ok(FieldShape::Fixed(StructField {
                    name: StructFieldName::new(name),
                    data_type,
                }))
            }
            type_shape => Ok(FieldShape::InProcess {
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
}
