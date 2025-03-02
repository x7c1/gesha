use gesha_rust_types::{DocComments, SerdeAttribute, TypeHeader};
use heck::ToUpperCamelCase;
use openapi_types::v3_0::{ComponentName, SchemaObject};

#[derive(Clone, Debug)]
pub struct TypeHeaderShape {
    pub name: ComponentName,
    pub doc_comments: Option<DocComments>,
    pub is_nullable: bool,
    pub serde_attrs: Vec<SerdeAttribute>,
    _hide_default_constructor: bool,
}

impl TypeHeaderShape {
    pub fn new(
        name: impl Into<String>,
        object: &impl HeaderPartsGenerator,
        serde_attrs: Vec<SerdeAttribute>,
    ) -> Self {
        let name = {
            let camel_cased = name.into().to_upper_camel_case();
            ComponentName::new(camel_cased)
        };
        let parts = object.generate();
        Self {
            name,
            doc_comments: to_doc_comments(parts.title.as_deref(), parts.description.as_deref()),
            is_nullable: parts.nullable.unwrap_or(false),
            serde_attrs,
            _hide_default_constructor: true,
        }
    }

    pub fn from_name(name: impl Into<String>) -> Self {
        let name = {
            let camel_cased = name.into().to_upper_camel_case();
            ComponentName::new(camel_cased)
        };
        Self {
            name,
            doc_comments: None,
            is_nullable: false,
            serde_attrs: vec![],
            _hide_default_constructor: true,
        }
    }

    pub fn define(self) -> TypeHeader {
        TypeHeader::new(self.name, self.doc_comments, self.serde_attrs)
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

pub trait HeaderPartsGenerator {
    fn generate(&self) -> HeaderParts;
}

impl HeaderPartsGenerator for SchemaObject {
    fn generate(&self) -> HeaderParts {
        HeaderParts {
            title: self.title.clone(),
            description: self.description.clone(),
            nullable: self.nullable,
        }
    }
}

pub struct HeaderParts {
    pub title: Option<String>,
    pub description: Option<String>,
    pub nullable: Option<bool>,
}
