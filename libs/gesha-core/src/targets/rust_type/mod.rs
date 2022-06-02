use indexmap::IndexMap;
use std::fmt::{Debug, Display, Formatter};

pub type Modules = IndexMap<ModuleName, Vec<Definition>>;

#[derive(Debug, Hash, Eq, PartialEq)]
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

#[derive(Debug)]
pub enum Definition {
    StructDef(StructDef),
    VecDef(VecDef),
}

#[derive(Debug)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<StructField>,
}

impl From<StructDef> for Definition {
    fn from(x: StructDef) -> Self {
        Self::StructDef(x)
    }
}

#[derive(Debug)]
pub struct VecDef {
    pub name: String,
    pub type_name: String,
}

impl From<VecDef> for Definition {
    fn from(x: VecDef) -> Self {
        Self::VecDef(x)
    }
}

#[derive(Debug)]
pub struct StructField {
    pub name: String,
    pub data_type: FieldType,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    String,
    Int32,
    Int64,
    // TODO: include type parameter
    Vec,
    Option(Box<FieldType>),
}
