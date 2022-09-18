use crate::targets::rust_type::{Definition, DeriveAttribute, TypeHeader};

#[derive(Clone, Debug)]
pub struct RequestBodyDef {
    pub header: TypeHeader,
    pub derive_attrs: Vec<DeriveAttribute>,
    _hide_default_constructor: bool,
}

impl RequestBodyDef {
    pub fn new(header: TypeHeader) -> Self {
        Self {
            header,
            derive_attrs: DeriveAttribute::all(),
            _hide_default_constructor: true,
        }
    }
}

impl From<RequestBodyDef> for Definition {
    fn from(this: RequestBodyDef) -> Self {
        Self::RequestBodyDef(this)
    }
}
