mod data_type;
pub use data_type::DataType;

mod definition;
pub use definition::{Definition, EnumDef, NewTypeDef, PresetDef, StructDef};

mod derive_attribute;
pub use derive_attribute::DeriveAttribute;

mod enum_variant;
pub use enum_variant::{EnumVariant, EnumVariantAttribute, EnumVariantName};

mod modules;
pub use modules::Modules;

mod request_body_def;
pub use request_body_def::RequestBodyDef;

mod struct_field;
pub use struct_field::{StructField, StructFieldAttribute};

mod struct_field_name;
pub use struct_field_name::StructFieldName;

use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub struct Module {
    pub name: ModuleName,
    pub definitions: Vec<Definition>,
    pub use_statements: Vec<UseStatement>,
    _hide_default_constructor: bool,
}

impl Module {
    pub fn new(
        name: ModuleName,
        definitions: Vec<Definition>,
        use_statements: Vec<UseStatement>,
    ) -> Self {
        Self {
            name,
            definitions,
            use_statements,
            _hide_default_constructor: true,
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ModuleName(String);

impl ModuleName {
    pub fn new<A: Into<String>>(a: A) -> Self {
        Self(a.into())
    }
}

impl Display for ModuleName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug)]
pub struct TypeHeader {
    pub name: String,
    pub doc_comments: DocComments,
    _hide_default_constructor: bool,
}

impl TypeHeader {
    pub fn new<A: Into<String>>(name: A, doc_comments: DocComments) -> Self {
        Self {
            name: name.into(),
            doc_comments,
            _hide_default_constructor: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DocComments(Option<String>);

impl DocComments {
    pub fn wrap(this: Option<String>) -> Self {
        Self(this.map(|text| format!("/**\n{text}\n*/\n")))
    }
}

impl Display for DocComments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(text) => Display::fmt(text, f),
            None => Ok(()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UseStatement(String);

impl UseStatement {
    pub fn new<A: Into<String>>(a: A) -> Self {
        Self(a.into())
    }
}

impl From<UseStatement> for String {
    fn from(x: UseStatement) -> Self {
        x.0
    }
}
