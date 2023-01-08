use crate::targets::rust_type::{DocComments, TypeHeader};
use openapi_types::v3_0::{ComponentName, SchemaObject};

#[derive(Clone, Debug)]
pub struct TypeHeaderShape {
    pub name: ComponentName,
    pub doc_comments: DocComments,
    pub is_nullable: bool,
}

impl TypeHeaderShape {
    pub fn new(name: ComponentName, object: &SchemaObject) -> Self {
        Self {
            name,
            doc_comments: to_doc_comments(object.title.as_deref(), object.description.as_deref()),
            is_nullable: object.nullable.unwrap_or(false),
        }
    }
    pub fn define(self) -> TypeHeader {
        TypeHeader::new(self.name, self.doc_comments)
    }
}

fn to_doc_comments(title: Option<&str>, description: Option<&str>) -> DocComments {
    let trim = |x: &str| x.trim().to_string();
    let maybe = match (title.map(trim), description.map(trim)) {
        (t, None) => t,
        (None, d) => d,
        (t, d) if t == d => t,
        (Some(t), Some(d)) => Some(format!("{t}\n\n{d}")),
    };
    DocComments::wrap(maybe)
}
