use crate::misc::TryMap;
use crate::v3_0::components::schemas::{DefinitionShape, RefShape, TypeHeaderShape, TypeShape};
use gesha_core::conversions::Result;
use gesha_rust_types::{EnumDef, EnumVariant, EnumVariantAttribute, EnumVariantName};
use openapi_types::v3_0::EnumValues;

#[derive(Clone, Debug)]
pub struct EnumShape {
    pub header: TypeHeaderShape,
    pub variants: Vec<EnumVariantShape>,
}

impl EnumShape {
    pub fn new(header: TypeHeaderShape, values: EnumValues) -> Self {
        Self {
            header,
            variants: values.into_iter().map(to_enum_variant).collect(),
        }
    }

    pub fn map_type(mut self, f: impl Fn(TypeShape) -> Result<TypeShape>) -> Result<Self> {
        self.variants = self.variants.try_map(|x| x.map_type(&f))?;
        Ok(self)
    }

    pub fn define(self) -> Result<EnumDef> {
        let variants = self.variants.try_map(|x| x.define())?;
        let def = EnumDef::new(self.header.define(), variants);
        Ok(def)
    }
}

impl From<EnumShape> for DefinitionShape {
    fn from(this: EnumShape) -> Self {
        Self::Enum(this)
    }
}

fn to_enum_variant(original: String) -> EnumVariantShape {
    let name = EnumVariantName::new(original.as_str());
    let mut attrs = vec![];
    if name.as_str() != original {
        attrs.push(EnumVariantAttribute::new(format!(
            r#"serde(rename="{original}")"#
        )))
    }
    EnumVariantShape {
        name,
        attributes: attrs,
        case: EnumCaseShape::Unit,
    }
}

#[derive(Clone, Debug)]
pub struct EnumVariantShape {
    pub name: EnumVariantName,
    pub attributes: Vec<EnumVariantAttribute>,
    pub case: EnumCaseShape,
}

impl EnumVariantShape {
    pub fn tuple(
        name: EnumVariantName,
        types: Vec<RefShape>,
        attributes: Vec<EnumVariantAttribute>,
    ) -> Result<Self> {
        let types = types.into_iter().map(|shape| shape.into()).collect();
        Ok(EnumVariantShape {
            name,
            attributes,
            case: EnumCaseShape::Tuple(types),
        })
    }

    pub fn map_type(mut self, f: impl Fn(TypeShape) -> Result<TypeShape>) -> Result<Self> {
        self.case = match self.case {
            EnumCaseShape::Unit => self.case,
            EnumCaseShape::Tuple(types) => {
                let types = types.try_map(f)?;
                EnumCaseShape::Tuple(types)
            }
        };
        Ok(self)
    }

    pub fn define(self) -> Result<EnumVariant> {
        let variant = match self.case {
            EnumCaseShape::Unit => EnumVariant::unit(self.name, self.attributes),
            EnumCaseShape::Tuple(xs) => {
                let types = xs.try_map(|x| x.define())?;
                EnumVariant::tuple(self.name, types, self.attributes)
            }
        };
        Ok(variant)
    }
}

#[derive(Clone, Debug)]
pub enum EnumCaseShape {
    Unit,
    Tuple(Vec<TypeShape>),
}
