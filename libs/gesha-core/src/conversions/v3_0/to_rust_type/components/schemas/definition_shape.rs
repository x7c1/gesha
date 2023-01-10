use crate::conversions::v3_0::to_rust_type::components::schemas::{
    AllOfShape, FieldShape, ModShape, StructShape, TypeHeaderShape, TypeShape,
};
use crate::conversions::Error::PostProcessBroken;
use crate::conversions::Result;
use crate::targets::rust_type::{
    Definition, EnumDef, EnumVariant, EnumVariantAttribute, EnumVariantName, NewTypeDef, StructDef,
    StructField,
};
use openapi_types::v3_0::{ComponentName, EnumValues};

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
    pub fn as_type_definition(&self) -> Option<TypeDefinitionShape> {
        match self {
            Self::Struct(shape) => Some(TypeDefinitionShape {
                type_header: &shape.header,
                fields: Some(&shape.fields),
            }),
            Self::NewType { header, .. } | Self::Enum { header, .. } => Some(TypeDefinitionShape {
                type_header: header,
                fields: None,
            }),
            // TODO: return here to merge multiple allOf
            Self::AllOf { .. } => None,
            Self::Mod { .. } => None,
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

pub struct TypeDefinitionShape<'a> {
    type_header: &'a TypeHeaderShape,
    fields: Option<&'a Vec<FieldShape>>,
}

impl TypeDefinitionShape<'_> {
    pub fn type_name(&self) -> &ComponentName {
        &self.type_header.name
    }

    pub fn is_type_name(&self, name: &str) -> bool {
        self.type_name().as_ref() == name
    }

    pub fn is_nullable(&self) -> bool {
        self.type_header.is_nullable
    }

    pub fn field_shapes(&self) -> Option<&Vec<FieldShape>> {
        self.fields
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
