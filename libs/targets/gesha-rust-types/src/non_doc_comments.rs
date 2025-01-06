use std::fmt::{Display, Formatter};

/// https://doc.rust-lang.org/reference/comments.html#non-doc-comments
#[derive(Clone, Debug, PartialEq)]
pub struct NonDocComments(String);

impl NonDocComments {
    pub fn block(text: impl Into<String>) -> Self {
        let text = text.into();
        NonDocComments(format!("/*\n{text}\n*/\n"))
    }
}

impl Display for NonDocComments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
