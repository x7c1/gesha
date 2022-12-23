use crate::targets::rust_type::{
    DataType, Definitions, DeriveAttribute, EnumVariant, ErrorDef, Imports, MediaTypeDef, Module,
    ModuleName, RequestBodyDef, StructField, TypeHeader,
};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
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
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum PresetDef {
    Error(ErrorDef),
    /// rf. https://stackoverflow.com/q/44331037
    Patch,
    MediaType(MediaTypeDef),
    FromJson,
}

impl From<PresetDef> for Definition {
    fn from(this: PresetDef) -> Self {
        Definition::PresetDef(this)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ModDef {
    // TODO:
    pub defs: Vec<Definition>,
}

impl From<ModDef> for Module {
    fn from(this: ModDef) -> Self {
        Module::new(
            ModuleName::new("todo"),
            Definitions::from_iter(this.defs),
            Imports::new(),
        )
    }
}

impl From<ModDef> for Definition {
    fn from(this: ModDef) -> Self {
        Self::ModDef(this)
    }
}
