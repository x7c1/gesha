use gesha_rust_types::{DeriveAttribute, DocComments, SerdeAttribute, TypeHeader, TypeIdentifier};
use openapi_types::v3_0::SchemaObject;

#[derive(Clone, Debug)]
pub struct TypeHeaderShape {
    pub name: TypeIdentifier,
    pub doc_comments: Option<DocComments>,
    pub is_nullable: bool,
    pub serde_attrs: Vec<SerdeAttribute>,
    pub derive_attrs: Vec<DeriveAttribute>,
    _hide_default_constructor: bool,
}

impl TypeHeaderShape {
    pub fn new(
        name: TypeIdentifier,
        body: impl Into<HeaderBody>,
        serde_attrs: Vec<SerdeAttribute>,
    ) -> Self {
        let body = body.into();
        Self {
            name,
            doc_comments: to_doc_comments(body.title.as_deref(), body.description.as_deref()),
            is_nullable: body.nullable.unwrap_or(false),
            serde_attrs,
            // TODO
            derive_attrs: DeriveAttribute::all(),
            _hide_default_constructor: true,
        }
    }

    pub fn from_name(name: TypeIdentifier) -> Self {
        Self {
            name,
            doc_comments: None,
            is_nullable: false,
            serde_attrs: vec![],
            // TODO
            derive_attrs: DeriveAttribute::all(),
            _hide_default_constructor: true,
        }
    }

    pub fn define(self) -> TypeHeader {
        TypeHeader::new(
            self.name,
            self.doc_comments,
            self.serde_attrs,
            self.derive_attrs,
        )
    }
}

fn to_doc_comments(title: Option<&str>, description: Option<&str>) -> Option<DocComments> {
    let trim = |x: &str| x.trim().to_string();
    let maybe = match (title.map(trim), description.map(trim)) {
        (t, None) => t,
        (None, d) => d,
        (t, d) if t == d => t,
        (Some(t), Some(d)) => Some(format!("{t}\n\n{d}")),
    };
    DocComments::wrap(maybe)
}

pub struct HeaderBody {
    pub title: Option<String>,
    pub description: Option<String>,
    pub nullable: Option<bool>,
}

impl From<&SchemaObject> for HeaderBody {
    fn from(value: &SchemaObject) -> Self {
        Self {
            title: value.title.clone(),
            description: value.description.clone(),
            nullable: value.nullable,
        }
    }
}
