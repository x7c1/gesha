use crate::{Definition, EnumMacroForFrom, EnumMacroForSerde, EnumVariant, TypeHeader};

#[derive(Clone, Debug, PartialEq)]
pub struct EnumDef {
    pub header: TypeHeader,
    pub variants: Vec<EnumVariant>,
    pub macro_for_serde: Option<EnumMacroForSerde>,
    pub macro_for_from: Option<EnumMacroForFrom>,
    _hide_default_constructor: bool,
}

impl EnumDef {
    pub fn new(
        header: TypeHeader,
        variants: Vec<EnumVariant>,
        macro_for_serde: Option<EnumMacroForSerde>,
        macro_for_from: Option<EnumMacroForFrom>,
    ) -> Self {
        Self {
            header,
            variants,
            macro_for_serde,
            macro_for_from,
            _hide_default_constructor: true,
        }
    }
    pub fn symbol_name(&self) -> &str {
        self.header.name.as_ref()
    }
}

impl From<EnumDef> for Definition {
    fn from(this: EnumDef) -> Self {
        Self::EnumDef(Box::new(this))
    }
}
