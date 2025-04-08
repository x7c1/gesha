use crate::{EnumConstant, EnumVariantName};
use indexmap::IndexMap;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct EnumMacroVariants(IndexMap<EnumVariantName, EnumConstant>);

impl EnumMacroVariants {
    pub fn insert(&mut self, name: EnumVariantName, constant: EnumConstant) {
        self.0.insert(name, constant);
    }
    pub fn iter(&self) -> impl Iterator<Item = (&EnumVariantName, &EnumConstant)> {
        self.0.iter()
    }
}
