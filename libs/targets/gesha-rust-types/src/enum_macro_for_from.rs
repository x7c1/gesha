use crate::{EnumConstant, EnumMacroVariants, TypeIdentifier};
use std::fmt::{Display, Formatter};

/**
Rendered as follows:
```ignore
gesha_macros::impl_enum_from!(
    IntEnum,
    super::core::Error,
    [i32],
    [(_0, 0), (_100, 100), (_200, 200)],
);
```

From the following YAML:
```yaml
IntEnum:
  type: integer
  format: int32
  enum:
    - 0
    - 100
    - 200
*/
#[derive(Clone, Debug, PartialEq)]
pub struct EnumMacroForFrom {
    pub name: TypeIdentifier,
    pub types: Vec<EnumMacroTypeForFrom>,
    pub variants: EnumMacroVariants,

    /// e.g. depth = 3 for `super::super::super::core::Error`.
    pub depth: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnumMacroTypeForFrom {
    I32,
    I64,
    // TODO: support other types like Str, Bool.
}

impl EnumMacroTypeForFrom {
    pub fn from_constant(constant: &EnumConstant) -> Option<Self> {
        match constant {
            EnumConstant::I32(_) => Some(Self::I32),
            EnumConstant::I64(_) => Some(Self::I64),
            _ => None,
        }
    }
}

impl Display for EnumMacroTypeForFrom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnumMacroTypeForFrom::I32 => Display::fmt("i32", f),
            EnumMacroTypeForFrom::I64 => Display::fmt("i64", f),
        }
    }
}
