use crate::targets::rust_type::{DeriveAttribute, DocComments};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct TypeHeader {
    pub name: String,
    pub derive_attrs: Vec<DeriveAttribute>,
    pub doc_comments: DocComments,
    _hide_default_constructor: bool,
}

impl TypeHeader {
    pub fn new<A: Into<String>>(name: A, doc_comments: DocComments) -> Self {
        Self {
            name: name.into(),
            derive_attrs: DeriveAttribute::all(),
            doc_comments,
            _hide_default_constructor: true,
        }
    }
}
