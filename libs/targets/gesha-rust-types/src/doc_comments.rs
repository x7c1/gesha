use std::fmt::{Display, Formatter};

// TODO: unwrap option
/// https://doc.rust-lang.org/reference/comments.html#doc-comments
#[derive(Clone, Debug, PartialEq)]
pub struct DocComments(Option<String>);

impl DocComments {
    pub fn wrap(this: Option<String>) -> Self {
        Self(this.map(|text| format!("/**\n{text}\n*/\n")))
    }
}

impl Display for DocComments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(text) => Display::fmt(text, f),
            None => Ok(()),
        }
    }
}
