use crate::{DataType, EnumDef, ModDef, NewTypeDef, PresetDef, RequestBodyDef, StructDef};

#[derive(Clone, Debug, PartialEq)]
pub enum Definition {
    StructDef(StructDef),
    NewTypeDef(NewTypeDef),
    EnumDef(EnumDef),
    PresetDef(PresetDef),
    RequestBodyDef(RequestBodyDef),
    ModDef(ModDef),
}

impl Definition {
    pub fn any_type<F>(&self, f: F) -> bool
    where
        F: Fn(&DataType) -> bool,
    {
        match self {
            Definition::StructDef(x) => x.fields.iter().any(|x| f(&x.data_type)),
            Definition::NewTypeDef(x) => f(&x.data_type),
            Definition::EnumDef(_) => false,
            Definition::PresetDef(_) => false,
            Definition::RequestBodyDef(_) => false,
            Definition::ModDef(_) => false,
        }
    }

    /// Return the symbol name of the definition.
    ///
    /// e.g.
    /// - struct Foo -> "Foo"
    /// - mod bar -> "bar"
    pub fn symbol_name(&self) -> &str {
        match self {
            Definition::StructDef(x) => x.symbol_name(),
            Definition::NewTypeDef(x) => x.symbol_name(),
            Definition::EnumDef(x) => x.symbol_name(),
            Definition::PresetDef(x) => x.symbol_name(),
            Definition::RequestBodyDef(x) => x.symbol_name(),
            Definition::ModDef(x) => x.symbol_name(),
        }
    }
}
