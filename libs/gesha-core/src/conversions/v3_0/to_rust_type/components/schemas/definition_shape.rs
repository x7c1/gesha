use crate::conversions::v3_0::to_rust_type::components::schemas::{
    AllOfShape, FieldShape, ModShape, StructShape, TypeHeaderShape, TypeShape,
};
use crate::conversions::Error::PostProcessBroken;
use crate::conversions::Result;
use crate::targets::rust_type::{
    Definition, EnumDef, EnumVariant, EnumVariantAttribute, EnumVariantName, NewTypeDef, StructDef,
    StructField,
};
use openapi_types::v3_0::{EnumValues, ReferenceObject, SchemaObject};

#[derive(Clone, Debug)]
pub enum DefinitionShape {
    AllOf(AllOfShape),
    Struct(StructShape),
    NewType {
        header: TypeHeaderShape,
        type_shape: TypeShape,
    },
    Enum {
        header: TypeHeaderShape,
        values: EnumValues,
    },
    Mod(ModShape),
}

impl DefinitionShape {
    pub fn type_header(&self) -> Option<&TypeHeaderShape> {
        match self {
            Self::AllOf(shape) => Some(&shape.header),
            Self::Struct(shape) => Some(&shape.header),
            Self::NewType { header, .. } => Some(header),
            Self::Enum { header, .. } => Some(header),
            Self::Mod(_) => None,
        }
    }

    pub fn any_type(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        match self {
            Self::AllOf(x) => x.any_type(f),
            Self::Struct(x) => x.any_type(f),
            Self::NewType { type_shape, .. } => f(type_shape),
            Self::Enum { .. } => false,
            Self::Mod(x) => x.any_type(f),
        }
    }

    pub fn any_type_directly(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        match self {
            Self::AllOf(x) => x.any_type_directly(f),
            Self::Struct(x) => x.any_type_directly(f),
            Self::NewType { type_shape, .. } => f(type_shape),
            Self::Enum { .. } => false,
            Self::Mod(_) => false,
        }
    }

    pub fn collect_fields(
        &self,
        f: impl Fn(&ReferenceObject<SchemaObject>) -> Vec<FieldShape>,
    ) -> Vec<FieldShape> {
        match self {
            Self::Struct(shape) => shape.fields.clone(),
            Self::AllOf(shape) => shape.collect_fields(f),
            Self::NewType { .. } | Self::Enum { .. } | Self::Mod(_) => vec![],
        }
    }

    pub fn define(self) -> Result<Definition> {
        match self {
            Self::Struct(StructShape { header, fields }) => {
                let def = StructDef::new(header.define(), define_fields(fields)?);
                Ok(def.into())
            }
            Self::NewType { header, type_shape } => {
                let def = NewTypeDef::new(header.define(), type_shape.define()?);
                Ok(def.into())
            }
            Self::Enum { header, values } => {
                let variants = values.into_iter().map(to_enum_variant).collect();
                let def = EnumDef::new(header.define(), variants);
                Ok(def.into())
            }
            Self::Mod(x) => x.define().map(|x| x.into()),
            Self::AllOf { .. } => Err(PostProcessBroken {
                detail: format!(
                    "'allOf' must be processed before 'to_definitions'.\n{:#?}",
                    self
                ),
            }),
        }
    }
}

impl TryFrom<DefinitionShape> for Definition {
    type Error = crate::conversions::Error;

    fn try_from(this: DefinitionShape) -> Result<Self> {
        this.define()
    }
}

fn define_fields(shapes: Vec<FieldShape>) -> Result<Vec<StructField>> {
    shapes.into_iter().map(|field| field.define()).collect()
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
