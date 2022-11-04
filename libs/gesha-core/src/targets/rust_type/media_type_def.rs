use indexmap::IndexMap;

#[derive(Clone, Debug)]
pub struct MediaTypeDef {
    /// e.g. "ApplicationJson" -> "application/json"
    pub translator: IndexMap<&'static str, &'static str>,
}

// TODO: use EnumVariantName instead of &str
// TODO: define MediaTypeValue
