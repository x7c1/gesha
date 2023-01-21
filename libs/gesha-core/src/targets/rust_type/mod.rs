mod data_type;
pub use data_type::DataType;

mod definition;
pub use definition::{Definition, EnumDef, ModDef, NewTypeDef, PresetDef, StructDef};

mod definitions;
pub use definitions::Definitions;

mod derive_attribute;
pub use derive_attribute::DeriveAttribute;

mod enum_variant;
pub use enum_variant::{EnumCase, EnumVariant, EnumVariantAttribute};

mod enum_variant_name;
pub use enum_variant_name::EnumVariantName;

mod error_def;
pub use error_def::{ErrorDef, ErrorVariant};

mod imports;
pub use imports::{Imports, Package};

mod media_type_def;
pub use media_type_def::MediaTypeDef;

mod modules;
pub use modules::Modules;

mod request_body_def;
pub use request_body_def::{MediaTypeVariant, MediaTypeVariants, RequestBodyDef};

mod struct_field;
pub use struct_field::{StructField, StructFieldAttribute};

mod struct_field_name;
pub use struct_field_name::StructFieldName;

mod type_header;
pub use type_header::TypeHeader;

use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ModuleName(String);

impl ModuleName {
    pub fn new<A: Into<String>>(a: A) -> Self {
        Self(a.into())
    }
}

impl Display for ModuleName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct DocComments(Option<String>);

impl DocComments {
    pub fn wrap(this: Option<String>) -> Self {
        Self(this.map(|text| format!("/**\n{text}\n*/\n")))
    }
}

impl Display for DocComments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(text) => Display::fmt(text, f),
            None => Ok(()),
        }
    }
}
