mod definition;
pub use definition::{Definition, EnumDef, EnumVariant, NewTypeDef, PresetDef, StructDef};

mod derive_attribute;
pub use derive_attribute::DeriveAttribute;

mod modules;
pub use modules::Modules;

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
    pub fn new(this: Option<String>) -> Self {
        Self(this)
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
pub struct StructField {
    pub name: StructFieldName,
    pub data_type: DataType,
}

#[derive(Clone, Debug)]
pub enum DataType {
    Bool,
    Int32,
    Int64,
    Float32,
    Float64,
    Option(Box<DataType>),
    Patch(Box<DataType>),
    String,
    Vec(Box<DataType>),
    Custom(String),
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = String::from(self.clone());
        Display::fmt(&x, f)
    }
}

impl From<DataType> for String {
    fn from(x: DataType) -> Self {
        match x {
            DataType::Bool => "bool".to_string(),
            DataType::Int32 => "i32".to_string(),
            DataType::Int64 => "i64".to_string(),
            DataType::Float32 => "f32".to_string(),
            DataType::Float64 => "f64".to_string(),
            DataType::Option(x) => format!("Option<{}>", String::from(*x)),
            DataType::Patch(x) => format!("Patch<{}>", String::from(*x)),
            DataType::String => "String".to_string(),
            DataType::Vec(x) => format!("Vec<{}>", String::from(*x)),
            DataType::Custom(x) => x,
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
