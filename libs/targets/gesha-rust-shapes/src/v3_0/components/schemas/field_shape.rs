use crate::v3_0::components::schemas::{Optionality, TypeShape, to_doc_comments};
use gesha_collections::default::default;
use gesha_collections::partial_result::MergeOps;
use gesha_collections::tracking::WithContextOps;
use gesha_core::conversions::{Output, Result};
use gesha_rust_types::{DocComments, StructField, StructFieldAttribute, StructFieldName};
use openapi_types::v3_0::{
    ComponentName, RequiredSchemaFields, SchemaCase, SchemaObject, SchemaProperties,
};

#[derive(Clone, Debug)]
pub struct FieldShape {
    pub name: ComponentName,
    pub type_shape: TypeShape,
    pub doc_comments: Option<DocComments>,
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
        let field = StructField::new(name, data_type, attrs, self.doc_comments);
        Ok(field)
    }

    pub fn override_by(mut self, target: FieldShape) -> Self {
        let (Some(current), Some(next)) = (
            self.type_shape.get_optionality(),
            target.type_shape.get_optionality(),
        ) else {
            return self;
        };
        let optionality = Optionality {
            // cannot change 'required' once set to true
            is_required: current.is_required || next.is_required,
            // can change 'nullable' at any time
            is_nullable: next.is_nullable,
        };
        self.type_shape = target.type_shape.set_optionality(optionality);
        self
    }

    pub fn erase_doc_comments(&mut self) {
        self.doc_comments = None;
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
        let (title, description) = title_and_description(&case);
        let doc_comments = to_doc_comments(title, description);
        Ok(FieldShape {
            name: name.clone(),
            type_shape: TypeShape::from_case(case, is_required).with_context(name)?,
            doc_comments,
        })
    }

    fn is_required(&self, name: &ComponentName) -> bool {
        match &self.required {
            Some(required) => required.contains(name.as_ref()),
            None => false,
        }
    }
}

fn title_and_description(case: &SchemaCase) -> (Option<&str>, Option<&str>) {
    match case {
        SchemaCase::Schema(object) => (object.title.as_deref(), object.description.as_deref()),
        SchemaCase::Reference(_) => default(),
    }
}
