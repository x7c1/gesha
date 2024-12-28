use std::fmt::{Display, Formatter};

/// https://doc.rust-lang.org/reference/comments.html#non-doc-comments
#[derive(Clone, Debug, PartialEq)]
pub struct NonDocComments(String);

impl NonDocComments {
    pub fn wrap(this: Option<String>) -> Option<Self> {
        this.map(|text| format!("/*\n{text}\n*/\n")).map(Self)
    }
}

impl Display for NonDocComments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
