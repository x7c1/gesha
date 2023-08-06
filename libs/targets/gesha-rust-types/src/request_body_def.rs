use crate::{Definition, EnumVariant, TypeHeader};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
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

impl From<MediaTypeVariants> for Vec<EnumVariant> {
    fn from(this: MediaTypeVariants) -> Self {
        this.0.into_iter().map(|x| x.variant).collect()
    }
}

impl IntoIterator for MediaTypeVariants {
    type Item = <Vec<MediaTypeVariant> as IntoIterator>::Item;
    type IntoIter = <Vec<MediaTypeVariant> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
