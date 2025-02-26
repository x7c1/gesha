use crate::v3_0::components::schemas::TypeShape;
use gesha_core::conversions::{Output, Result};
use gesha_rust_types::{StructField, StructFieldAttribute, StructFieldName};
use openapi_types::core::OutputMergeOps;
use openapi_types::v3_0::{
    ComponentName, RequiredSchemaFields, SchemaCase, SchemaObject, SchemaProperties,
};

#[derive(Clone, Debug)]
pub struct FieldShape {
    pub name: ComponentName,
    pub type_shape: TypeShape,
}

impl FieldShape {
    pub fn from_object(object: SchemaObject) -> Output<Vec<Self>> {
        Self::from_properties(object.properties, object.required)
    }

    pub fn from_object_ref(object: &SchemaObject) -> Output<Vec<Self>> {
        Self::from_properties(object.properties.clone(), object.required.clone())
    }

    pub fn any_type(xs: &[Self], f: &impl Fn(&TypeShape) -> bool) -> bool {
        xs.iter().any(|x| f(&x.type_shape))
    }

    pub fn define(self) -> Result<StructField> {
        let name = StructFieldName::new(self.name.as_ref());
        let attrs = self.create_field_attrs(&name);
        let data_type = self.type_shape.define()?;
        let field = StructField::new(name, data_type, attrs);
        Ok(field)
    }

    fn from_properties(
        properties: Option<SchemaProperties>,
        required: Option<RequiredSchemaFields>,
    ) -> Output<Vec<Self>> {
        let to_field_shapes = |props| ToFieldShapes { required }.apply(props);
        properties.map(to_field_shapes).merge()
    }

    fn create_field_attrs(&self, name: &StructFieldName) -> Vec<StructFieldAttribute> {
        let mut attrs = vec![];

        let original = &self.name;
        if original.as_ref() != name.as_str() {
            attrs.push(StructFieldAttribute::new(format!(
                r#"serde(rename="{original}")"#
            )));
        }
        if matches!(self.type_shape, TypeShape::Maybe(..)) {
            attrs.push(StructFieldAttribute::new(
                r#"serde(default, skip_serializing_if = "Option::is_none")"#,
            ))
        }
        if matches!(self.type_shape, TypeShape::Patch(..)) {
            attrs.push(StructFieldAttribute::new(
                r#"serde(default, skip_serializing_if = "Patch::is_absent")"#,
            ));
        }
        attrs
    }
}

/// SchemaProperties -> Vec<FieldShape>
struct ToFieldShapes {
    required: Option<RequiredSchemaFields>,
}

impl ToFieldShapes {
    fn apply(self, props: SchemaProperties) -> Output<Vec<FieldShape>> {
        props
            .into_iter()
            .map(|(name, case)| self.to_field(name, case))
            .collect::<Vec<Result<_>>>()
            .merge()
    }

    fn to_field(&self, name: ComponentName, case: SchemaCase) -> Result<FieldShape> {
        let is_required = self.is_required(&name);
        Ok(FieldShape {
            name: name.clone(),
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
