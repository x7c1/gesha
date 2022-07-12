use crate::targets::rust_type::{DataType, DeriveAttribute, StructField, TypeHeader};
use heck::ToUpperCamelCase;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub enum Definition {
    StructDef(StructDef),
    NewTypeDef(NewTypeDef),
    EnumDef(EnumDef),
    PresetDef(PresetDef),
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
        }
    }
}

#[derive(Clone, Debug)]
pub enum PresetDef {
    Patch(String),
}

impl PresetDef {
    pub fn patch() -> Self {
        let code = include_str!("patch.rs.tpl");
        Self::Patch(code.to_string())
    }
}

impl Display for PresetDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PresetDef::Patch(x) => Display::fmt(x, f),
        }
    }
}

impl From<PresetDef> for Definition {
    fn from(this: PresetDef) -> Self {
        Definition::PresetDef(this)
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
