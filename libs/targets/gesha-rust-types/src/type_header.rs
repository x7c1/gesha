use crate::{DeriveAttribute, DocComments, SerdeAttribute};

#[derive(Clone, Debug, PartialEq)]
pub struct TypeHeader {
    // TODO: use TypeIdentifier
    pub name: String,
    pub derive_attrs: Vec<DeriveAttribute>,
    pub serde_attrs: Vec<SerdeAttribute>,
    pub doc_comments: Option<DocComments>,
    _hide_default_constructor: bool,
}

impl TypeHeader {
    pub fn new<A: Into<String>>(
        name: A,
        doc_comments: Option<DocComments>,
        serde_attrs: Vec<SerdeAttribute>,
        derive_attrs: Vec<DeriveAttribute>,
    ) -> Self {
        Self {
            name: name.into(),
            derive_attrs,
            serde_attrs,
            doc_comments,
            _hide_default_constructor: true,
        }
    }
}
