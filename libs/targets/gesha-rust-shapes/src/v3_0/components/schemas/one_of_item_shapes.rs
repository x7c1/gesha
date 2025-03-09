use crate::v3_0::components::schemas::{OneOfItemShape, RefShape};
use gesha_core::conversions::Output;
use openapi_types::core::OutputMergeOps;
use openapi_types::v3_0::SchemaCase;

#[derive(Clone, Debug, Default)]
pub struct OneOfItemShapes(Vec<OneOfItemShape>);

impl OneOfItemShapes {
    pub fn from_schema_cases(cases: Vec<SchemaCase>) -> Output<Self> {
        let inner = cases
            .into_iter()
            .map(OneOfItemShape::from_schema_case)
            .collect::<Vec<_>>()
            .merge()
            .map(|outputs| outputs.merge())
            .merge();

        inner.map(Self)
    }

    pub fn new(items: Vec<OneOfItemShape>) -> Self {
        Self(items)
    }

    pub fn head_if_single_ref(&self) -> Option<&RefShape> {
        let [OneOfItemShape { target }] = self.0.as_slice() else {
            return None;
        };
        Some(target)
    }

    pub fn into_vec(self) -> Vec<OneOfItemShape> {
        self.0
    }
}
