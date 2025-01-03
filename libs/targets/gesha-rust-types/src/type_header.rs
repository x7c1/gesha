use crate::{DeriveAttribute, DocComments, SerdeAttribute};

#[derive(Clone, Debug, PartialEq)]
pub struct TypeHeader {
    pub name: String,
    pub derive_attrs: Vec<DeriveAttribute>,
    pub serde_attrs: Vec<SerdeAttribute>,
    pub doc_comments: DocComments,
    _hide_default_constructor: bool,
}

impl TypeHeader {
    pub fn new<A: Into<String>>(
        name: A,
        doc_comments: DocComments,
        serde_attrs: Vec<SerdeAttribute>,
    ) -> Self {
        Self {
            name: name.into(),
            derive_attrs: DeriveAttribute::all(),
            serde_attrs,
            doc_comments,
            _hide_default_constructor: true,
        }
    }
}
