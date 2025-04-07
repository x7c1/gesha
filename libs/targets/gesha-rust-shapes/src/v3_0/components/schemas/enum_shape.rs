use crate::v3_0::components::schemas::{DefinitionShape, RefShape, TypeHeaderShape, TypeShape};
use gesha_collections::seq::TryMapOps;
use gesha_core::conversions::Error::EnumFormatMismatch;
use gesha_core::conversions::Result;
use gesha_rust_types::{
    EnumConstant, EnumDef, EnumMacroForSerde, EnumVariant, EnumVariantAttribute, EnumVariantName,
};
use openapi_types::v3_0::{EnumValue, EnumValues, FormatModifier};

#[derive(Clone, Debug)]
pub struct EnumShape {
    pub header: TypeHeaderShape,
    pub variants: Vec<EnumVariantShape>,
    pub macro_for_serde: Option<EnumMacroForSerde>,
    pub format: Option<FormatModifier>,
}

impl EnumShape {
    pub fn new(
        header: TypeHeaderShape,
        values: EnumValues,
        format: Option<FormatModifier>,
    ) -> Result<Self> {
        Ok(Self {
            header,
            variants: values.try_map(|value| to_enum_variant(value, &format))?,
            macro_for_serde: None,
            format,
        })
    }

    pub fn map_type(mut self, f: impl Fn(TypeShape) -> Result<TypeShape>) -> Result<Self> {
        self.variants = self.variants.try_map(|x| x.map_type(&f))?;
        Ok(self)
    }

    pub fn define(self) -> Result<EnumDef> {
        let variants = self.variants.try_map(|x| x.define())?;
        let def = EnumDef::new(self.header.define(), variants, self.macro_for_serde);
        Ok(def)
    }
}

impl From<EnumShape> for DefinitionShape {
    fn from(this: EnumShape) -> Self {
        Self::Enum(this)
    }
}

fn to_enum_variant(
    original: EnumValue,
    format: &Option<FormatModifier>,
) -> Result<EnumVariantShape> {
    let original_name = to_enum_variant_name(&original);
    let name = EnumVariantName::new(&original_name)?;
    let mut attrs = vec![];
    if name.as_str() != original_name {
        attrs.push(EnumVariantAttribute::new(format!(
            r#"serde(rename="{original_name}")"#
        )))
    }
    let constant = to_enum_constant(original, format)?;
    Ok(EnumVariantShape {
        name,
        attributes: attrs,
        case: EnumCaseShape::Unit(constant),
    })
}

fn to_enum_constant(value: EnumValue, format: &Option<FormatModifier>) -> Result<EnumConstant> {
    type V = EnumValue;
    let constant = match (value, format) {
        (V::String(value), _) => EnumConstant::Str(value),
        (V::Integer(value), _) if value > 0 => EnumConstant::U64(value as u64),
        (V::Integer(value), _) => from_signed_int_value(value, format)?,
        (V::Boolean(value), _) => EnumConstant::Bool(value),
        (V::Null, _) => EnumConstant::Null,
    };
    Ok(constant)
}

fn from_signed_int_value(value: i64, format: &Option<FormatModifier>) -> Result<EnumConstant> {
    let constant = match (value, format) {
        (value, Some(FormatModifier::Int32)) => {
            let x = i32::try_from(value).map_err(|_| EnumFormatMismatch {
                format: FormatModifier::Int32.to_string(),
                value: value.to_string(),
            })?;
            EnumConstant::I32(x)
        }
        (value, Some(FormatModifier::Int64)) => EnumConstant::I64(value),
        (value, _) => EnumConstant::I64(value),
    };
    Ok(constant)
}

#[test]
fn test_from_signed_int_value() {
    let target_value = (i32::MAX as i64) + 1;
    let target_format = FormatModifier::Int32;

    let EnumFormatMismatch { format, value } =
        from_signed_int_value(target_value, &Some(target_format.clone())).unwrap_err()
    else {
        panic!("Missing expected error")
    };
    assert_eq!(format, target_format.as_ref());
    assert_eq!(value, target_value.to_string());
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
