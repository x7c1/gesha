use crate::conversions::v3_0::to_rust_type::components::schemas::{
    DefinitionShape, TypeHeaderShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::{EnumDef, EnumVariant, EnumVariantAttribute, EnumVariantName};
use openapi_types::v3_0::EnumValues;

#[derive(Clone, Debug)]
pub struct EnumShape {
    pub header: TypeHeaderShape,
    pub variants: EnumVariantsShape,
}

impl EnumShape {
    pub fn define(self) -> Result<EnumDef> {
        match self.variants {
            EnumVariantsShape::Unit(values) => {
                let variants = values.into_iter().map(to_enum_variant).collect();
                let def = EnumDef::new(self.header.define(), variants);
                Ok(def)
            }
            EnumVariantsShape::Tuple => {
                todo!()
            }
        }
    }
}

impl From<EnumShape> for DefinitionShape {
    fn from(this: EnumShape) -> Self {
        Self::Enum(this)
    }
}

#[derive(Clone, Debug)]
pub enum EnumVariantsShape {
    Unit(EnumValues),
    Tuple,
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
