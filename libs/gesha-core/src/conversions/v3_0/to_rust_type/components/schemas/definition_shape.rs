use crate::broken;
use crate::conversions::v3_0::to_rust_type::components::schemas::{
    AllOfShape, EnumShape, FieldShape, ModShape, OneOfShape, Ref, StructShape, TypeHeaderShape,
    TypeShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::{
    Definition, EnumDef, EnumVariant, EnumVariantAttribute, EnumVariantName, NewTypeDef, StructDef,
    StructField,
};
use openapi_types::v3_0::EnumValues;

#[derive(Clone, Debug)]
pub enum DefinitionShape {
    AllOf(AllOfShape),
    Enum(EnumShape),
    Mod(ModShape),
    NewType {
        header: TypeHeaderShape,
        type_shape: TypeShape,
    },
    OneOf(OneOfShape),
    Struct(StructShape),
}

impl DefinitionShape {
    pub fn type_header(&self) -> Option<&TypeHeaderShape> {
        match self {
            Self::AllOf(shape) => Some(&shape.header),
            Self::Struct(shape) => Some(&shape.header),
            Self::NewType { header, .. } => Some(header),
            Self::Enum(shape) => Some(&shape.header),
            Self::OneOf(_) => todo!(),
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
            Self::OneOf(x) => x.any_type(f),
        }
    }

    pub fn any_type_directly(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        match self {
            Self::AllOf(x) => x.any_type_directly(f),
            Self::Struct(x) => x.any_type_directly(f),
            Self::NewType { type_shape, .. } => f(type_shape),
            Self::OneOf(x) => x.any_type_directly(f),
            Self::Enum { .. } => false,
            Self::Mod(_) => false,
        }
    }

    pub fn collect_fields(&self, resolve_ref: impl Fn(&Ref) -> Vec<FieldShape>) -> Vec<FieldShape> {
        match self {
            Self::Struct(shape) => shape.fields.clone(),
            Self::AllOf(shape) => shape.expand_fields(resolve_ref),
            Self::NewType { .. } | Self::Enum { .. } | Self::Mod(_) => vec![],
            Self::OneOf(_) => todo!(),
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
            Self::Enum(x) => x.define().map(|x| x.into()),
            Self::Mod(x) => x.define().map(|x| x.into()),
            Self::OneOf(x) => x.define().map(|x| x.into()),
            Self::AllOf(_) => Err(broken!(self)),
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
