use crate::conversions::v3_0::to_rust_type::components::schemas::{
    DefinitionShape, Ref, TypeHeaderShape, TypeShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::{EnumDef, EnumVariant, EnumVariantAttribute, EnumVariantName};
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
        self.variants = self
            .variants
            .into_iter()
            .map(|variant| variant.map_type(&f))
            .collect::<Result<Vec<_>>>()?;

        Ok(self)
    }

    pub fn define(self) -> Result<EnumDef> {
        let variants = self
            .variants
            .into_iter()
            .map(|x| x.define())
            .collect::<Result<Vec<_>>>()?;

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
        types: Vec<Ref>,
        attributes: Vec<EnumVariantAttribute>,
    ) -> Self {
        let types = types
            .into_iter()
            .map(|x| TypeShape::Ref {
                target: x,
                is_required: true,
            })
            .collect();

        EnumVariantShape {
            name,
            attributes,
            case: EnumCaseShape::Tuple(types),
        }
    }

    pub fn map_type(mut self, f: impl Fn(TypeShape) -> Result<TypeShape>) -> Result<Self> {
        self.case = match self.case {
            EnumCaseShape::Unit => self.case,
            EnumCaseShape::Tuple(types) => {
                let types = types.into_iter().map(f).collect::<Result<Vec<_>>>()?;
                EnumCaseShape::Tuple(types)
            }
        };
        Ok(self)
    }

    pub fn define(self) -> Result<EnumVariant> {
        let variant = match self.case {
            EnumCaseShape::Unit => EnumVariant::unit(self.name, self.attributes),
            EnumCaseShape::Tuple(xs) => {
                let types = xs
                    .into_iter()
                    .map(|x| x.define())
                    .collect::<Result<Vec<_>>>()?;

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
