use crate::v3_0::components::schemas::{DefinitionShape, FieldShape, RefShape};
use gesha_core::conversions::{Output, Result};
use openapi_types::core::OutputMergeOps;
use openapi_types::v3_0::{SchemaCase, SchemaObject};

#[derive(Clone, Debug)]
pub enum AllOfItemShape {
    Object(Vec<FieldShape>),
    Ref(RefShape),
}

impl AllOfItemShape {
    pub fn from_schema_cases(cases: Vec<SchemaCase>) -> Output<Vec<Self>> {
        cases
            .into_iter()
            .map(Self::from_schema_case)
            .collect::<Vec<_>>()
            .merge()
            .map(|x| x.merge())
            .merge()
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
        resolve_ref: impl Fn(&RefShape) -> Vec<FieldShape>,
    ) -> Vec<FieldShape> {
        match self {
            Self::Object(x) => x.clone(),
            Self::Ref(x) => resolve_ref(x),
        }
    }

    fn from_schema_object(object: SchemaObject) -> Output<Self> {
        let items = FieldShape::from_object(object);
        items.map(Self::Object)
    }

    fn from_schema_case(case: SchemaCase) -> Result<Output<Self>> {
        let output = match case {
            SchemaCase::Schema(object) => Self::from_schema_object(*object),
            SchemaCase::Reference(object) => {
                let shape = RefShape::new(object, /* is_required */ true)?;
                Output::ok(Self::Ref(shape))
            }
        };
        Ok(output)
    }
}
