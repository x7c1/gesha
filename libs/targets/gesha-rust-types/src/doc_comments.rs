use std::fmt::{Display, Formatter};

/// https://doc.rust-lang.org/reference/comments.html#doc-comments
#[derive(Clone, Debug, PartialEq)]
pub struct DocComments(String);

impl DocComments {
    pub fn wrap(this: Option<String>) -> Option<Self> {
        this.map(|text| Self(format!("/**\n{text}\n*/\n")))
    }
}

impl Display for DocComments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
