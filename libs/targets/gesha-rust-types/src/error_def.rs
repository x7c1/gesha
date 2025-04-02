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
    pub fn iter(&self) -> impl Iterator<Item = &ErrorVariant> {
        self.0.iter()
    }
    pub fn symbol_name(&self) -> &str {
        "Error"
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ErrorVariant {
    InvalidJson,
    UnsupportedMediaType,
}
