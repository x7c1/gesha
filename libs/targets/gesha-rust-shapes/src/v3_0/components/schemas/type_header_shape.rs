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
    // pub fn new(
    //     name: impl Into<String>,
    //     object: &SchemaObject,
    //     serde_attrs: Vec<SerdeAttribute>,
    // ) -> Self {
    //     let name = {
    //         let camel_cased = name.into().to_upper_camel_case();
    //         ComponentName::new(camel_cased)
    //     };
    //     Self {
    //         name,
    //         doc_comments: to_doc_comments(object.title.as_deref(), object.description.as_deref()),
    //         is_nullable: object.nullable.unwrap_or(false),
    //         serde_attrs,
    //         _hide_default_constructor: true,
    //     }
    // }
    pub fn new(
        name: impl Into<String>,
        object: &impl CanConstructHeader,
        serde_attrs: Vec<SerdeAttribute>,
    ) -> Self {
        let name = {
            let camel_cased = name.into().to_upper_camel_case();
            ComponentName::new(camel_cased)
        };
        Self {
            name,
            doc_comments: to_doc_comments(object.title(), object.description()),
            is_nullable: object.nullable().unwrap_or(false),
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

pub trait CanConstructHeader {
    fn title(&self) -> Option<&str>;
    fn description(&self) -> Option<&str>;
    fn nullable(&self) -> Option<bool>;
}

impl CanConstructHeader for SchemaObject {
    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn nullable(&self) -> Option<bool> {
        self.nullable
    }
}
