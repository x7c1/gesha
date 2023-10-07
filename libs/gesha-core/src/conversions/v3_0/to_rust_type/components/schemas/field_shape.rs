use crate::conversions::v3_0::to_rust_type::components::schemas::TypeShape;
use crate::conversions::v3_0::to_rust_type::is_patch;
use crate::conversions::Result;
use gesha_rust_types::{DataType, StructField, StructFieldAttribute, StructFieldName};
use openapi_types::v3_0::{
    ComponentName, RequiredSchemaFields, SchemaCase, SchemaObject, SchemaProperties,
};

#[derive(Clone, Debug)]
pub struct FieldShape {
    pub name: ComponentName,
    pub type_shape: TypeShape,
}

impl FieldShape {
    pub fn from_object(object: SchemaObject) -> Result<Vec<Self>> {
        Self::from_properties(object.properties, object.required)
    }

    pub fn from_object_ref(object: &SchemaObject) -> Result<Vec<Self>> {
        Self::from_properties(object.properties.clone(), object.required.clone())
    }

    pub fn any_type(xs: &[Self], f: &impl Fn(&TypeShape) -> bool) -> bool {
        xs.iter().any(|x| f(&x.type_shape))
    }

    pub fn define(self) -> Result<StructField> {
        let data_type = self.type_shape.define()?;
        let name = StructFieldName::new(self.name.as_ref());
        let attrs = to_field_attrs(&self.name, &name, &data_type);
        let field = StructField::new(name, data_type, attrs);
        Ok(field)
    }

    fn from_properties(
        properties: Option<SchemaProperties>,
        required: Option<RequiredSchemaFields>,
    ) -> Result<Vec<Self>> {
        let to_field_shapes = |props| ToFieldShapes { required }.apply(props);
        properties.map(to_field_shapes).unwrap_or(Ok(vec![]))
    }
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

    fn to_field(&self, name: ComponentName, case: SchemaCase) -> Result<FieldShape> {
        let is_required = self.is_required(&name);
        Ok(FieldShape {
            name,
            type_shape: TypeShape::from_case(case, is_required)?,
        })
    }

    fn is_required(&self, name: &ComponentName) -> bool {
        match &self.required {
            Some(required) => required.contains(name.as_ref()),
            None => false,
        }
    }
}

fn to_field_attrs(
    original: &ComponentName,
    name: &StructFieldName,
    tpe: &DataType,
) -> Vec<StructFieldAttribute> {
    let mut attrs = vec![];
    if original.as_ref() != name.as_str() {
        attrs.push(StructFieldAttribute::new(format!(
            r#"serde(rename="{original}")"#
        )));
    }
    if is_patch(tpe) {
        attrs.push(StructFieldAttribute::new(
            r#"serde(default, skip_serializing_if = "Patch::is_absent")"#,
        ));
    }
    attrs
}
