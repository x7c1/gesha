mod derive_attribute;
pub use derive_attribute::DeriveAttribute;

mod modules;
pub use modules::Modules;

mod struct_field_name;
pub use struct_field_name::StructFieldName;

use heck::ToUpperCamelCase;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub struct Module {
    pub name: ModuleName,
    pub definitions: Vec<Definition>,
    pub use_statements: Vec<UseStatement>,
    _hide_default_constructor: bool,
}

impl Module {
    pub fn new(name: ModuleName, definitions: Vec<Definition>) -> Self {
        Self {
            name,
            definitions,
            use_statements: vec![
                UseStatement::new("serde::Deserialize"),
                UseStatement::new("serde::Serialize"),
            ],
            _hide_default_constructor: false,
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
pub enum Definition {
    StructDef(StructDef),
    NewTypeDef(NewTypeDef),
    EnumDef(EnumDef),
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
pub struct StructDef {
    pub header: TypeHeader,
    pub fields: Vec<StructField>,
    pub derive_attrs: Vec<DeriveAttribute>,
    _hide_default_constructor: bool,
}

impl StructDef {
    pub fn new(header: TypeHeader, fields: Vec<StructField>) -> Self {
        Self {
            header,
            fields,
            derive_attrs: DeriveAttribute::all(),
            _hide_default_constructor: true,
        }
    }
}

impl From<StructDef> for Definition {
    fn from(x: StructDef) -> Self {
        Self::StructDef(x)
    }
}

#[derive(Clone, Debug)]
pub struct NewTypeDef {
    pub header: TypeHeader,
    pub data_type: DataType,
    pub derive_attrs: Vec<DeriveAttribute>,
    _hide_default_constructor: bool,
}

impl NewTypeDef {
    pub fn new(header: TypeHeader, data_type: DataType) -> Self {
        Self {
            header,
            data_type,
            derive_attrs: DeriveAttribute::all(),
            _hide_default_constructor: true,
        }
    }
}

impl From<NewTypeDef> for Definition {
    fn from(x: NewTypeDef) -> Self {
        Self::NewTypeDef(x)
    }
}

#[derive(Clone, Debug)]
pub struct EnumDef {
    pub header: TypeHeader,
    pub variants: Vec<EnumVariant>,
    pub derive_attrs: Vec<DeriveAttribute>,
    _hide_default_constructor: bool,
}

impl EnumDef {
    pub fn new(header: TypeHeader, variants: Vec<EnumVariant>) -> Self {
        Self {
            header,
            variants,
            derive_attrs: DeriveAttribute::all(),
            _hide_default_constructor: true,
        }
    }
}

impl From<EnumDef> for Definition {
    fn from(this: EnumDef) -> Self {
        Self::EnumDef(this)
    }
}

#[derive(Clone, Debug)]
pub struct EnumVariant(String);

impl EnumVariant {
    pub fn new(x: String) -> Self {
        // TODO: replace x with Rust compatible chars if illegal chars are included.
        EnumVariant(x)
    }

    pub fn to_upper_camel(&self) -> String {
        self.0.to_upper_camel_case()
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
