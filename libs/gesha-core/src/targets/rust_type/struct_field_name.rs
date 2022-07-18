use heck::ToSnakeCase;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct StructFieldName(String);

impl StructFieldName {
    pub fn new(x: &str) -> Self {
        Self(to_rust_compatible_name(x))
    }
    pub fn as_str(&self) -> &str {
        &self.0
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
fn to_rust_compatible_name(target: &str) -> String {
    let target = target.to_snake_case();

    // TODO: include other keywords
    ["break", "continue", "ref", "type"]
        .into_iter()
        .find(|x| &target == x)
        .map(|x| x.to_string() + "_")
        .unwrap_or(target)
}
