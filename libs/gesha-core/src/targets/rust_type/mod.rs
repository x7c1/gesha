mod derive_attribute;
pub use derive_attribute::DeriveAttribute;

mod struct_field_name;
pub use struct_field_name::StructFieldName;

use heck::ToUpperCamelCase;
use indexmap::IndexMap;
use std::fmt::{Debug, Display, Formatter};

pub type Modules = IndexMap<ModuleName, Vec<Definition>>;

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
pub struct StructDef {
    pub name: String,
    pub fields: Vec<StructField>,
    pub derive_attrs: Vec<DeriveAttribute>,
    _hide_default_constructor: bool,
}

impl StructDef {
    pub fn new<A: Into<String>>(name: A, fields: Vec<StructField>) -> Self {
        Self {
            name: name.into(),
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
    pub name: String,
    pub data_type: DataType,
    pub derive_attrs: Vec<DeriveAttribute>,
    _hide_default_constructor: bool,
}

impl NewTypeDef {
    pub fn new<A: Into<String>>(name: A, data_type: DataType) -> Self {
        Self {
            name: name.into(),
            data_type,
            derive_attrs: DeriveAttribute::all(),
            _hide_default_constructor: false
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
    pub name: String,
    pub variants: Vec<EnumVariant>,
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
