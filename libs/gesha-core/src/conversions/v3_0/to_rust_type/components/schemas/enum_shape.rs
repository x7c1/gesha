use crate::conversions::v3_0::to_rust_type::components::schemas::{
    DefinitionShape, TypeHeaderShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::{EnumDef, EnumVariant, EnumVariantAttribute, EnumVariantName};
use openapi_types::v3_0::EnumValues;

#[derive(Clone, Debug)]
pub struct EnumShape {
    pub header: TypeHeaderShape,
    pub variants: Vec<EnumVariant>,
}

impl EnumShape {
    pub fn new(header: TypeHeaderShape, values: EnumValues) -> Self {
        Self {
            header,
            variants: values.into_iter().map(to_enum_variant).collect(),
        }
    }

    pub fn define(self) -> Result<EnumDef> {
        let def = EnumDef::new(self.header.define(), self.variants);
        Ok(def)
    }
}

impl From<EnumShape> for DefinitionShape {
    fn from(this: EnumShape) -> Self {
        Self::Enum(this)
    }
}

fn to_enum_variant(original: String) -> EnumVariant {
    let name = EnumVariantName::new(original.as_str());
    let mut attrs = vec![];
    if name.as_str() != original {
        attrs.push(EnumVariantAttribute::new(format!(
            r#"serde(rename="{original}")"#
        )))
    }
    EnumVariant::unit(name, attrs)
}
