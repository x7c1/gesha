use crate::{Definition, EnumMacroSerdeImpl, EnumVariant, TypeHeader};

#[derive(Clone, Debug, PartialEq)]
pub struct EnumDef {
    pub header: TypeHeader,
    pub variants: Vec<EnumVariant>,
    pub macro_serde_impl: Option<EnumMacroSerdeImpl>,
    _hide_default_constructor: bool,
}

impl EnumDef {
    pub fn new(
        header: TypeHeader,
        variants: Vec<EnumVariant>,
        macro_serde_impl: Option<EnumMacroSerdeImpl>,
    ) -> Self {
        Self {
            header,
            macro_serde_impl,
            variants,
            _hide_default_constructor: true,
        }
    }
    pub fn symbol_name(&self) -> &str {
        self.header.name.as_ref()
    }
}

impl From<EnumDef> for Definition {
    fn from(this: EnumDef) -> Self {
        Self::EnumDef(this)
    }
}
