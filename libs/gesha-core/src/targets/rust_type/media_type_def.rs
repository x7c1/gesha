use crate::targets::rust_type::EnumVariantName;
use indexmap::IndexMap;

#[derive(Clone, Debug)]
pub struct MediaTypeDef {
    /// e.g. "ApplicationJson" -> "application/json"
    pub translator: IndexMap<EnumVariantName, &'static str>,
}
