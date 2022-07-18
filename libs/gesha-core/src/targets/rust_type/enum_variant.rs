use heck::ToUpperCamelCase;

#[derive(Clone, Debug)]
pub struct EnumVariant {
    pub compatible: String,
    pub original: String,
    _hide_default_constructor: bool,
}

impl EnumVariant {
    pub fn new(original: String) -> Self {
        // TODO: replace original with Rust compatible chars if illegal chars are included.
        EnumVariant {
            compatible: original.to_upper_camel_case(),
            original,
            _hide_default_constructor: true,
        }
    }
}
