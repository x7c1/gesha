use crate::{Definition, ErrorDef, MediaTypeDef};

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
