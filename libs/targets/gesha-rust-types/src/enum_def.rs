use crate::{Definition, EnumMacroImpl, EnumVariant, TypeHeader};

#[derive(Clone, Debug, PartialEq)]
pub struct EnumDef {
    pub header: TypeHeader,
    pub variants: Vec<EnumVariant>,
    pub macro_impl: Option<EnumMacroImpl>,
    _hide_default_constructor: bool,
}

impl EnumDef {
    pub fn new(
        header: TypeHeader,
        variants: Vec<EnumVariant>,
        macro_impl: Option<EnumMacroImpl>,
    ) -> Self {
        Self {
            header,
            macro_impl,
            variants,
            _hide_default_constructor: true,
        }
    }
}

impl From<EnumDef> for Definition {
    fn from(this: EnumDef) -> Self {
        Self::EnumDef(this)
    }
}
