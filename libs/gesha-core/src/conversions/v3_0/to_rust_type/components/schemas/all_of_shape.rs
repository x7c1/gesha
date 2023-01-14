use crate::conversions::v3_0::to_rust_type::components::schemas::{
    AllOfItemShape, DefinitionShape, FieldShape, TypeHeaderShape, TypeShape,
};
use openapi_types::v3_0::{ReferenceObject, RequiredSchemaFields, SchemaObject};

#[derive(Clone, Debug)]
pub struct AllOfShape {
    pub header: TypeHeaderShape,
    pub items: Vec<AllOfItemShape>,
    pub required: Option<RequiredSchemaFields>,
}

impl AllOfShape {
    pub fn any_type(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        self.items.iter().any(|item| match item {
            AllOfItemShape::Object(x) => FieldShape::any_type(&x.items, f),
            AllOfItemShape::Ref(_) => false,
        })
    }

    pub fn any_type_directly(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        self.items.iter().any(|item| match item {
            AllOfItemShape::Object(x) => x.items.iter().any(|x| f(&x.type_shape)),
            AllOfItemShape::Ref(_) => false,
        })
    }

    pub fn expand_fields(
        &self,
        resolve_ref: impl Fn(&ReferenceObject<SchemaObject>) -> Vec<FieldShape>,
    ) -> Vec<FieldShape> {
        let to_required = |mut field: FieldShape| {
            let is_required = self
                .required
                .as_ref()
                .map(|fields| fields.contains(field.name.as_ref()))
                .unwrap_or(false);

            if is_required {
                field.type_shape = field.type_shape.require();
            }
            field
        };
        self.items
            .iter()
            .flat_map(|x| x.collect_fields(&resolve_ref).into_iter().map(to_required))
            .collect()
    }
}

impl From<AllOfShape> for DefinitionShape {
    fn from(this: AllOfShape) -> Self {
        Self::AllOf(this)
    }
}
