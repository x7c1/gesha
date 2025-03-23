use crate::misc::TryMap;
use crate::v3_0::components::schemas::{DefinitionShape, RefShape, TypeHeaderShape, TypeShape};
use gesha_core::conversions::Result;
use gesha_rust_types::{
    EnumConstant, EnumDef, EnumMacroImpl, EnumVariant, EnumVariantAttribute, EnumVariantName,
};
use openapi_types::v3_0::{EnumValue, EnumValues};

#[derive(Clone, Debug)]
pub struct EnumShape {
    pub header: TypeHeaderShape,
    pub variants: Vec<EnumVariantShape>,
    pub macro_impl: Option<EnumMacroImpl>,
}

impl EnumShape {
    pub fn new(header: TypeHeaderShape, values: EnumValues) -> Self {
        Self {
            header,
            variants: values.into_iter().map(to_enum_variant).collect(),
            macro_impl: None,
        }
    }

    pub fn map_type(mut self, f: impl Fn(TypeShape) -> Result<TypeShape>) -> Result<Self> {
        self.variants = self.variants.try_map(|x| x.map_type(&f))?;
        Ok(self)
    }

    pub fn define(self) -> Result<EnumDef> {
        let variants = self.variants.try_map(|x| x.define())?;
        let def = EnumDef::new(self.header.define(), variants, self.macro_impl);
        Ok(def)
    }
}

impl From<EnumShape> for DefinitionShape {
    fn from(this: EnumShape) -> Self {
        Self::Enum(this)
    }
}

fn to_enum_variant(original: EnumValue) -> EnumVariantShape {
    let original_name = to_enum_variant_name(&original);
    let name = EnumVariantName::new(&original_name);
    let mut attrs = vec![];
    if name.as_str() != original_name {
        attrs.push(EnumVariantAttribute::new(format!(
            r#"serde(rename="{original_name}")"#
        )))
    }
    EnumVariantShape {
        name,
        attributes: attrs,
        case: EnumCaseShape::Unit(to_enum_constant(original)),
    }
}

fn to_enum_constant(value: EnumValue) -> EnumConstant {
    match value {
        EnumValue::String(value) => EnumConstant::Str(value),
        EnumValue::Integer(value) if value > 0 => EnumConstant::U64(value as u64),
        EnumValue::Integer(value) => EnumConstant::I64(value),
        EnumValue::Boolean(value) => EnumConstant::Bool(value),
        EnumValue::Null => EnumConstant::Null,
    }
}

fn to_enum_variant_name(value: &EnumValue) -> String {
    match value {
        EnumValue::String(value) => value.clone(),
        EnumValue::Integer(value) => value.to_string(),
        EnumValue::Boolean(value) => value.to_string(),

        /*
        This is not an issue, though it cannot distinguish between null and "null"
        because this library does not allow duplicate variant names.
        For example, in cases like "null" and null, or "123" and 123.
        */
        EnumValue::Null => "null".to_string(),
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
            EnumCaseShape::Unit(_) => self.case,
            EnumCaseShape::Tuple(types) => {
                let types = types.try_map(f)?;
                EnumCaseShape::Tuple(types)
            }
        };
        Ok(self)
    }

    pub fn define(self) -> Result<EnumVariant> {
        let variant = match self.case {
            EnumCaseShape::Unit(constant) => {
                EnumVariant::unit(self.name, constant, self.attributes)
            }
            EnumCaseShape::Tuple(xs) => {
                let types = xs.try_map(|x| x.define())?;
                EnumVariant::tuple(self.name, types, self.attributes)
            }
        };
        Ok(variant)
    }

    pub fn is_string(&self) -> bool {
        matches!(self.case, EnumCaseShape::Unit(EnumConstant::Str(_)))
    }
    pub fn is_tuple(&self) -> bool {
        matches!(self.case, EnumCaseShape::Tuple(_))
    }

    pub fn erase_attributes(&mut self) {
        self.attributes = vec![];
    }
}

#[derive(Clone, Debug)]
pub enum EnumCaseShape {
    Unit(EnumConstant),
    Tuple(Vec<TypeShape>),
}
