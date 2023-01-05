use indexmap::IndexSet;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ErrorDef(IndexSet<ErrorVariant>);

impl ErrorDef {
    pub fn new() -> Self {
        ErrorDef(IndexSet::new())
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn set(&mut self, variant: ErrorVariant) {
        let _ = self.0.insert(variant);
    }
}

impl IntoIterator for ErrorDef {
    type Item = <IndexSet<ErrorVariant> as IntoIterator>::Item;
    type IntoIter = <IndexSet<ErrorVariant> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for ErrorDef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_items(self.0.iter(), state)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ErrorVariant {
    InvalidJson,
    UnsupportedMediaType,
}
