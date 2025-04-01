use crate::{Definition, ErrorDef, MediaTypeDef};

#[derive(Clone, Debug, PartialEq)]
pub enum PresetDef {
    Error(ErrorDef),
    /// rf. https://stackoverflow.com/q/44331037
    Patch,
    MediaType(MediaTypeDef),
    FromJson,
}

impl PresetDef {
    pub fn name(&self) -> &str {
        match self {
            PresetDef::Error(x) => x.name(),
            PresetDef::Patch => "Patch",
            PresetDef::MediaType(x) => x.name(),
            PresetDef::FromJson => "FromJson",
        }
    }
}

impl From<PresetDef> for Definition {
    fn from(this: PresetDef) -> Self {
        Definition::PresetDef(this)
    }
}
