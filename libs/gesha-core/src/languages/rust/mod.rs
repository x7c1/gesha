use indexmap::IndexMap;

pub type RustModules = IndexMap<ModuleName, Vec<Definition>>;

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
    pub type_name: String,
}
