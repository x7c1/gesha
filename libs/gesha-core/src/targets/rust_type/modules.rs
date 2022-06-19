use std::vec::IntoIter;
use crate::targets::rust_type::Module;

#[derive(Clone, Debug)]
pub struct Modules(Vec<Module>);

impl Modules {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn new(module: Vec<Module>) -> Self {
        Self(module)
    }
}

impl IntoIterator for Modules {
    type Item = Module;
    type IntoIter = IntoIter<Module>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
