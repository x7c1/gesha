use indexmap::IndexMap;

pub type Modules = IndexMap<ModuleName, Vec<Definition>>;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct ModuleName(String);

impl ModuleName {
    pub fn new<A: Into<String>>(a: A) -> Self {
        Self(a.into())
    }
}

#[derive(Debug)]
pub enum Definition {
    StructDef {
        name: String,
        fields: Vec<StructField>,
    },
    VecDef {
        name: String,
        type_name: String,
    },
}

#[derive(Debug)]
pub struct StructField {
    pub name: String,
    pub data_type: FieldType,
}

#[derive(Debug)]
pub enum FieldType {
    String,
    Int64,
    // TODO: include type parameter
    Vec,
}

pub trait ToRust<A>: Sized {
    fn apply(this: A) -> crate::Result<Self>;
}
