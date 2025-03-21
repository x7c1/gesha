use crate::{
    DataType, Definitions, EnumDef, ErrorDef, Imports, MediaTypeDef, ModuleName, RequestBodyDef,
    StructField, TypeHeader,
};

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
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct StructDef {
    pub header: TypeHeader,
    pub fields: Vec<StructField>,
    _hide_default_constructor: bool,
}

impl StructDef {
    pub fn new(header: TypeHeader, fields: Vec<StructField>) -> Self {
        Self {
            header,
            fields,
            _hide_default_constructor: true,
        }
    }
}

impl From<StructDef> for Definition {
    fn from(x: StructDef) -> Self {
        Self::StructDef(x)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NewTypeDef {
    pub header: TypeHeader,
    pub data_type: DataType,
    _hide_default_constructor: bool,
}

impl NewTypeDef {
    pub fn new(header: TypeHeader, data_type: DataType) -> Self {
        Self {
            header,
            data_type,
            _hide_default_constructor: true,
        }
    }
}

impl From<NewTypeDef> for Definition {
    fn from(x: NewTypeDef) -> Self {
        Self::NewTypeDef(x)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModDef {
    pub name: ModuleName,
    pub imports: Imports,
    pub defs: Definitions,
}

impl From<ModDef> for Definition {
    fn from(this: ModDef) -> Self {
        Self::ModDef(this)
    }
}
