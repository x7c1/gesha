use crate::misc::TryMap;
use crate::v3_0::components::schemas::{AllOfItemShape, RefShape};
use gesha_core::conversions::Output;
use gesha_core::conversions::Result;
use openapi_types::core::OutputMergeOps;
use openapi_types::v3_0::SchemaCase;

#[derive(Clone, Debug, Default)]
pub struct AllOfItemShapes(Vec<AllOfItemShape>);

impl AllOfItemShapes {
    pub fn from_schema_cases(cases: Vec<SchemaCase>) -> Output<Self> {
        let inner = cases
            .into_iter()
            .map(AllOfItemShape::from_schema_case)
            .collect::<Vec<_>>()
            .merge()
            .map(|outputs| outputs.merge())
            .merge();

        inner.map(Self)
    }

    pub fn new(items: Vec<AllOfItemShape>) -> Self {
        Self(items)
    }

    pub fn head_if_single_ref(&self) -> Option<&RefShape> {
        let [AllOfItemShape::Ref(target)] = self.0.as_slice() else {
            return None;
        };
        Some(target)
    }

    pub fn transform_items(
        self,
        f: impl Fn(AllOfItemShape) -> Result<AllOfItemShape>,
    ) -> Result<Self> {
        let items = self.0.try_map(f)?;
        Ok(Self::new(items))
    }

    pub fn into_vec(self) -> Vec<AllOfItemShape> {
        self.0
    }

    pub fn iter(&self) -> impl Iterator<Item = &AllOfItemShape> {
        self.0.iter()
    }
}
