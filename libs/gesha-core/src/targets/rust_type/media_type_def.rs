use crate::targets::rust_type::{hash_items, EnumVariantName};
use indexmap::IndexMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaTypeDef {
    /// e.g. "ApplicationJson" -> "application/json"
    pub translator: IndexMap<EnumVariantName, &'static str>,
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for MediaTypeDef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_items(self.translator.iter(), state)
    }
}
