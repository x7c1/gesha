use heck::ToSnakeCase;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct StructFieldName {
    compatible: String,
    original: String,
}

impl StructFieldName {
    pub(crate) fn find_to_rename(&self) -> Option<&str> {
        if self.compatible == self.original {
            None
        } else {
            Some(&self.original)
        }
    }
}

impl StructFieldName {
    pub fn new<A: Into<String>>(a: A) -> Self {
        let original = a.into();
        Self {
            compatible: to_rust_compatible_name(&original),
            original,
        }
    }
}

impl Display for StructFieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.compatible, f)
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
