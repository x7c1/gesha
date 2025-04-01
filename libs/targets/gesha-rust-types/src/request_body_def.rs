use crate::{Definition, EnumVariant, TypeHeader};

#[derive(Clone, Debug, PartialEq)]
pub struct RequestBodyDef {
    pub header: TypeHeader,
    pub variants: MediaTypeVariants,
    _hide_default_constructor: bool,
}

impl RequestBodyDef {
    pub fn new(header: TypeHeader, variants: Vec<MediaTypeVariant>) -> Self {
        Self {
            header,
            variants: MediaTypeVariants(variants),
            _hide_default_constructor: true,
        }
    }
    pub fn symbol_name(&self) -> &str {
        self.header.name.as_ref()
    }
}

impl From<RequestBodyDef> for Definition {
    fn from(this: RequestBodyDef) -> Self {
        Self::RequestBodyDef(this)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct MediaTypeVariant {
    /// e.g. "application/json"
    pub header_value: String,
    pub variant: EnumVariant,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct MediaTypeVariants(Vec<MediaTypeVariant>);

impl MediaTypeVariants {
    pub fn iter(&self) -> impl Iterator<Item = &MediaTypeVariant> {
        self.0.iter()
    }
}
