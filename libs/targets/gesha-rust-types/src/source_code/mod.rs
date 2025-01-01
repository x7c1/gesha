use crate::ModDef;

mod render_module;
use render_module::render_module;

use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct SourceCode(Vec<ModDef>);

impl SourceCode {
    pub fn empty() -> Self {
        Self(vec![])
    }
    pub fn new(mod_defs: Vec<ModDef>) -> Self {
        Self(mod_defs)
    }
}

impl Display for SourceCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .try_for_each(|module| render_module(f, module))
    }
}