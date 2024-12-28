mod render_module;
use render_module::render_module;

use crate::{ModDef, NonDocComments};
use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Modules {
    comments: Option<NonDocComments>,
    mod_defs: Vec<ModDef>,
}

impl Modules {
    pub fn empty() -> Self {
        Self {
            mod_defs: vec![],
            comments: None,
        }
    }
    pub fn new(comments: Option<NonDocComments>, mod_defs: Vec<ModDef>) -> Self {
        Self { comments, mod_defs }
    }
}

impl Display for Modules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(comments) = &self.comments {
            write!(f, "{}", comments)?;
        }
        self.mod_defs
            .iter()
            .try_for_each(|module| render_module(f, module))
    }
}
