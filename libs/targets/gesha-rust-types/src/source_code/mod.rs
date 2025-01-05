use crate::{ModDef, ModuleDeclarations, NonDocComments};

mod render_module;
use render_module::render_module;

use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct SourceCode {
    preamble: Option<NonDocComments>,
    mod_decls: ModuleDeclarations,
    mod_defs: Vec<ModDef>,
}

impl SourceCode {
    pub fn empty() -> Self {
        Self {
            preamble: None,
            mod_decls: ModuleDeclarations::empty(),
            mod_defs: vec![],
        }
    }
    pub fn set_preamble(mut self, preamble: NonDocComments) -> Self {
        self.preamble = Some(preamble);
        self
    }
    pub fn set_mod_decls(mut self, mod_decls: ModuleDeclarations) -> Self {
        self.mod_decls = mod_decls;
        self
    }
    pub fn set_mod_defs(mut self, mod_defs: Vec<ModDef>) -> Self {
        self.mod_defs = mod_defs;
        self
    }
}

impl Display for SourceCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(preamble) = self.preamble.as_ref() {
            Display::fmt(preamble, f)?;
        }
        Display::fmt(&self.mod_decls, f)?;

        self.mod_defs
            .iter()
            .try_for_each(|module| render_module(f, module))
    }
}
