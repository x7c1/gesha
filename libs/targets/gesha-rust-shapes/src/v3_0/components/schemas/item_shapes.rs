use crate::misc::TryMap;
use crate::v3_0::components::schemas::RefShape;
use gesha_collections::partial_result::MergeOps;
use gesha_core::conversions::{Output, Result};
use openapi_types::v3_0::SchemaCase;

/// Items defined in allOf and oneOf
#[derive(Clone, Debug)]
pub struct CaseItemShapes<A: CaseItem>(Vec<A>);

impl<A: CaseItem> CaseItemShapes<A> {
    pub fn from_schema_cases(cases: impl Into<Vec<SchemaCase>>) -> Output<Self> {
        let inner = cases
            .into()
            .into_iter()
            .map(A::from_schema_case)
            .collect::<Vec<_>>()
            .merge()
            .map(|outputs| outputs.merge())
            .merge();

        inner.map(Self)
    }

    pub fn new(items: Vec<A>) -> Self {
        Self(items)
    }

    pub fn empty() -> Self {
        Self::new(vec![])
    }

    pub fn head_if_single_ref(&self) -> Option<&RefShape> {
        match self.0.as_slice() {
            [x] => x.to_ref_shape(),
            _ => None,
        }
    }

    pub fn transform_items(self, f: impl Fn(A) -> Result<A>) -> Result<Self> {
        let items = self.0.try_map(f)?;
        Ok(Self::new(items))
    }

    pub fn into_vec(self) -> Vec<A> {
        self.0
    }

    pub fn iter(&self) -> impl Iterator<Item = &A> {
        self.0.iter()
    }
}

pub trait CaseItem: Sized {
    fn from_schema_case(case: SchemaCase) -> Result<Output<Self>>;
    fn to_ref_shape(&self) -> Option<&RefShape>;
}
