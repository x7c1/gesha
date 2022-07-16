use crate::targets::rust_type::{DataType, Definition, Module};
use std::vec::IntoIter;

#[derive(Clone, Debug)]
pub struct Modules(Vec<Module>);

impl Modules {
    pub fn any_type<F>(&self, f: F) -> bool
    where
        F: Fn(&DataType) -> bool,
    {
        self.any_def(|x| x.any_type(|y| f(y)))
    }

    pub fn any_def<F>(&self, f: F) -> bool
    where
        F: Fn(&Definition) -> bool,
    {
        self.0.iter().any(|module| {
            module.definitions.iter().any(|def| {
                // dummy comment to mute cargo-clippy
                f(def)
            })
        })
    }

    pub fn push(&mut self, module: Module) {
        self.0.push(module)
    }
}

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
