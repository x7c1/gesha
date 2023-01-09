use crate::targets::rust_type::ModDef;
use std::vec::IntoIter;

#[derive(Clone, Debug)]
pub struct Modules(Vec<ModDef>);

impl Modules {
    pub fn empty() -> Self {
        Self(vec![])
    }
}

impl IntoIterator for Modules {
    type Item = ModDef;
    type IntoIter = IntoIter<ModDef>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<ModDef> for Modules {
    fn from_iter<T: IntoIterator<Item = ModDef>>(iter: T) -> Self {
        let xs = iter.into_iter().collect();
        Self(xs)
    }
}
