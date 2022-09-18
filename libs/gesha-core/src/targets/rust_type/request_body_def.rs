use crate::targets::rust_type::{Definition, DeriveAttribute, EnumVariant, TypeHeader};

#[derive(Clone, Debug)]
pub struct RequestBodyDef {
    pub header: TypeHeader,
    pub derive_attrs: Vec<DeriveAttribute>,
    pub variants: Vec<EnumVariant>,
    _hide_default_constructor: bool,
}

impl RequestBodyDef {
    pub fn new(header: TypeHeader, variants: Vec<EnumVariant>) -> Self {
        Self {
            header,
            derive_attrs: DeriveAttribute::all(),
            variants,
            _hide_default_constructor: true,
        }
    }
}

impl From<RequestBodyDef> for Definition {
    fn from(this: RequestBodyDef) -> Self {
        Self::RequestBodyDef(this)
    }
}
