use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct StructFieldName(String);

impl StructFieldName {
    pub fn new<A: Into<String>>(a: A) -> Self {
        Self(to_rust_compatible_name(a.into()))
    }
}

impl Display for StructFieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

/// append '_' if given string is reserved keyword.
///
/// https://doc.rust-lang.org/reference/keywords.html
fn to_rust_compatible_name(x: String) -> String {
    // TODO: avoid other keywords
    ["type"]
        .into_iter()
        .find(|y| &x == y)
        .map(|y| y.to_string() + "_")
        .unwrap_or(x)
}
