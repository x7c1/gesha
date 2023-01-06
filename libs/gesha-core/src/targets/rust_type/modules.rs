use crate::targets::rust_type::{DataType, Definition, ModDef};
use std::vec::IntoIter;

#[derive(Clone, Debug)]
pub struct Modules(Vec<ModDef>);

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
            module.defs.iter().any(|def| {
                // dummy comment to mute cargo-clippy
                f(def)
            })
        })
    }

    pub fn push(&mut self, module: ModDef) {
        self.0.push(module)
    }
}

impl Modules {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn new(module: Vec<ModDef>) -> Self {
        Self(module)
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
