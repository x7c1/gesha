use crate::targets::rust_type::{Definition, DeriveAttribute, EnumVariant, TypeHeader};

#[derive(Clone, Debug)]
pub struct RequestBodyDef {
    pub header: TypeHeader,
    pub derive_attrs: Vec<DeriveAttribute>,
    pub variants: MediaTypeVariants,
    _hide_default_constructor: bool,
}

impl RequestBodyDef {
    pub fn new(header: TypeHeader, variants: Vec<MediaTypeVariant>) -> Self {
        Self {
            header,
            derive_attrs: DeriveAttribute::all(),
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

#[derive(Clone, Debug)]
pub struct MediaTypeVariant {
    /// e.g. "application/json"
    pub header_value: String,
    pub variant: EnumVariant,
}

#[derive(Clone, Debug)]
pub struct MediaTypeVariants(Vec<MediaTypeVariant>);

impl From<MediaTypeVariants> for Vec<EnumVariant> {
    fn from(this: MediaTypeVariants) -> Self {
        this.0.into_iter().map(|x| x.variant).collect()
    }
}
