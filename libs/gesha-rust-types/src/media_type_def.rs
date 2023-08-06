use crate::EnumVariantName;
use indexmap::IndexMap;

#[derive(Clone, Debug, PartialEq)]
pub struct MediaTypeDef {
    /// e.g. "ApplicationJson" -> "application/json"
    pub translator: IndexMap<EnumVariantName, String>,
}
