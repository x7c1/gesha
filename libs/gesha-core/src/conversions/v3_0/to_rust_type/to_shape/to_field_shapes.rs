use super::to_type_shape;
use crate::conversions::v3_0::to_rust_type::FieldShape;
use crate::conversions::Result;
use crate::targets::rust_type::StructFieldName;
use openapi_types::v3_0::{RequiredSchemaFields, SchemaCase, SchemaFieldName, SchemaProperties};

pub(super) fn to_field_shapes(
    properties: Option<SchemaProperties>,
    required: Option<RequiredSchemaFields>,
) -> Result<Vec<FieldShape>> {
    let to_field_shapes = |props| ToFieldShapes { required }.apply(props);
    properties.map(to_field_shapes).unwrap_or(Ok(vec![]))
}

/// SchemaProperties -> Vec<FieldShape>
struct ToFieldShapes {
    required: Option<RequiredSchemaFields>,
}

impl ToFieldShapes {
    fn apply(self, props: SchemaProperties) -> Result<Vec<FieldShape>> {
        props
            .into_iter()
            .map(|(name, case)| self.to_field(name, case))
            .collect()
    }

    fn to_field(&self, name: SchemaFieldName, case: SchemaCase) -> Result<FieldShape> {
        let is_required = self.is_required(&name);
        let type_shape = to_type_shape(case, is_required)?;
        Ok(FieldShape::InProcess {
            name: StructFieldName::new(name),
            type_shape,
        })
    }

    fn is_required(&self, name: &SchemaFieldName) -> bool {
        match &self.required {
            Some(required) => required.contains(name.as_ref()),
            None => false,
        }
    }
}
