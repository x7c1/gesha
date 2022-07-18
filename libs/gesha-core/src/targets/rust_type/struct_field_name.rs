use heck::ToSnakeCase;

#[derive(Clone, Debug)]
pub struct StructFieldName {
    pub compatible: String,
    pub original: String,
    _hide_default_constructor: bool,
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
            _hide_default_constructor: true,
        }
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
