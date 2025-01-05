use crate::{ModDef, NonDocComments};

mod render_module;
use render_module::render_module;

use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct SourceCode {
    preamble: Option<NonDocComments>,
    defs: Vec<ModDef>,
}

impl SourceCode {
    pub fn empty() -> Self {
        Self {
            preamble: None,
            defs: vec![],
        }
    }
    pub fn new(preamble: Option<NonDocComments>, mod_defs: Vec<ModDef>) -> Self {
        Self {
            preamble,
            defs: mod_defs,
        }
    }
}

impl Display for SourceCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(preamble) = self.preamble.as_ref() {
            Display::fmt(preamble, f)?;
        }
        self.defs
            .iter()
            .try_for_each(|module| render_module(f, module))
    }
}
