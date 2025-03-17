use crate::{DeriveAttribute, DocComments, SerdeAttribute, TypeIdentifier};

#[derive(Clone, Debug, PartialEq)]
pub struct TypeHeader {
    pub name: TypeIdentifier,
    pub derive_attrs: Vec<DeriveAttribute>,
    pub serde_attrs: Vec<SerdeAttribute>,
    pub doc_comments: Option<DocComments>,
    _hide_default_constructor: bool,
}

impl TypeHeader {
    pub fn new(
        name: TypeIdentifier,
        doc_comments: Option<DocComments>,
        serde_attrs: Vec<SerdeAttribute>,
        derive_attrs: Vec<DeriveAttribute>,
    ) -> Self {
        Self {
            name,
            derive_attrs,
            serde_attrs,
            doc_comments,
            _hide_default_constructor: true,
        }
    }
}
