use crate::conversions::v3_0::to_rust_type::components::schemas::{DefinitionShape, FieldShape};
use crate::conversions::Result;
use openapi_types::v3_0::{ReferenceObject, SchemaCase, SchemaObject};

#[derive(Clone, Debug)]
pub enum AllOfItemShape {
    Object(Vec<FieldShape>),
    Ref(ReferenceObject<SchemaObject>),
}

impl AllOfItemShape {
    pub fn from_schema_cases(cases: Vec<SchemaCase>) -> Result<Vec<Self>> {
        cases.into_iter().map(Self::from_schema_case).collect()
    }

    pub fn expand_fields<F>(self, f: F) -> Result<(Self, Vec<DefinitionShape>)>
    where
        F: Fn(Vec<FieldShape>) -> Result<(Vec<FieldShape>, Vec<DefinitionShape>)>,
    {
        match self {
            Self::Object(fields) => {
                let (fields, defs) = f(fields)?;
                Ok((Self::Object(fields), defs))
            }
            Self::Ref(_) => Ok((self, vec![])),
        }
    }

    pub fn collect_fields(
        &self,
        f: impl Fn(&ReferenceObject<SchemaObject>) -> Vec<FieldShape>,
    ) -> Vec<FieldShape> {
        match self {
            Self::Object(x) => x.clone(),
            Self::Ref(x) => f(x),
        }
    }

    fn from_schema_object(object: SchemaObject) -> Result<Self> {
        let shapes = FieldShape::from_object(object)?;
        Ok(Self::Object(shapes))
    }

    fn from_schema_case(case: SchemaCase) -> Result<Self> {
        let shape = match case {
            SchemaCase::Schema(object) => Self::from_schema_object(*object)?,
            SchemaCase::Reference(x) => Self::Ref(x),
        };
        Ok(shape)
    }
}
