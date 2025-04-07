use crate::{EnumCase, EnumConstant, EnumMacroVariants, EnumVariant, TypeIdentifier};
use indexmap::IndexMap;
use std::fmt::Display;

/**
Rendered as follows:
```ignore
gesha_macros::impl_enum_serde!(MixedTypeEnum {
    u64: [(_1000, 1000)],
    str: [(_2000, "2000"), (_2001, "2001"), (_2002, "2002")],
    i64: [(Minus42, -42)],
});
```

From the following YAML:
```yaml
MixedTypeEnum:
  enum:
    - 1000
    - "2000"
    - "2001"
    - "2002"
    - -42
```
*/
#[derive(Clone, Debug, PartialEq)]
pub struct EnumMacroForSerde {
    pub name: TypeIdentifier,
    pub type_variants: IndexMap<EnumMacroTypeForSerde, EnumMacroVariants>,
}

impl EnumMacroForSerde {
    pub fn from_variants(name: TypeIdentifier, variants: Vec<EnumVariant>) -> Self {
        let type_variants = variants
            .into_iter()
            .fold(IndexMap::new(), |mut map, variant| {
                let EnumCase::Unit(constant) = variant.case else {
                    return map;
                };
                let enum_type = EnumMacroTypeForSerde::from(&constant);
                let entry: &mut EnumMacroVariants = map.entry(enum_type).or_default();
                entry.insert(variant.name, constant);
                map
            });

        Self {
            name,
            type_variants,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EnumMacroTypeForSerde {
    Bool,
    I64,
    Null,
    Str,
    U64,
}

impl From<&EnumConstant> for EnumMacroTypeForSerde {
    fn from(value: &EnumConstant) -> Self {
        match value {
            EnumConstant::Bool(_) => Self::Bool,

            // `deserialize_any()` generated by `impl_enum_serde!` treats all positive values as u64.
            EnumConstant::I32(x) if *x > 0 => Self::U64,
            EnumConstant::I64(x) if *x > 0 => Self::U64,

            // `deserialize_any()` cannot tell i32 from i64.
            EnumConstant::I32(_) => Self::I64,
            EnumConstant::I64(_) => Self::I64,

            EnumConstant::Null => Self::Null,
            EnumConstant::Str(_) => Self::Str,
        }
    }
}

impl Display for EnumMacroTypeForSerde {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "bool"),
            Self::I64 => write!(f, "i64"),
            Self::Null => write!(f, "null"),
            Self::Str => write!(f, "str"),
            Self::U64 => write!(f, "u64"),
        }
    }
}
