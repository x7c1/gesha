use crate::v3_0::components::schemas::{
    AllOfShape, EnumShape, FieldShape, ModShape, NewTypeShape, OneOfShape, RefShape, StructShape,
    TypeHeaderShape, TypeShape,
};
use gesha_collections::seq::TryMap;
use gesha_core::broken;
use gesha_core::conversions::Result;
use gesha_rust_types::{Definition, DeriveAttribute, NewTypeDef, StructDef};

#[derive(Clone, Debug)]
pub enum DefinitionShape {
    AllOf(AllOfShape),
    Enum(EnumShape),
    Mod(ModShape),
    NewType(NewTypeShape),
    OneOf(OneOfShape),
    Struct(StructShape),
}

impl DefinitionShape {
    pub fn type_header(&self) -> Option<&TypeHeaderShape> {
        match self {
            Self::AllOf(shape) => Some(&shape.header),
            Self::Struct(shape) => Some(&shape.header),
            Self::NewType(shape) => Some(&shape.header),
            Self::Enum(shape) => Some(&shape.header),
            Self::OneOf(shape) => Some(&shape.header),
            Self::Mod(_) => None,
        }
    }

    pub fn any_type(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        match self {
            Self::AllOf(x) => x.any_type(f),
            Self::Struct(x) => x.any_type(f),
            Self::NewType(x) => f(&x.type_shape),
            Self::Enum { .. } => false,
            Self::Mod(x) => x.any_type(f),
            Self::OneOf(x) => x.any_type(f),
        }
    }

    pub fn any_type_directly(&self, f: &impl Fn(&TypeShape) -> bool) -> bool {
        match self {
            Self::AllOf(x) => x.any_type_directly(f),
            Self::Struct(x) => x.any_type_directly(f),
            Self::NewType(x) => f(&x.type_shape),
            Self::OneOf(x) => x.any_type_directly(f),
            Self::Enum { .. } => false,
            Self::Mod(_) => false,
        }
    }

    pub fn any_derive_directly(&self, f: impl FnMut(&DeriveAttribute) -> bool) -> bool {
        let header = match self {
            Self::AllOf(x) => &x.header,
            Self::Struct(x) => &x.header,
            Self::NewType(x) => &x.header,
            Self::OneOf(x) => &x.header,
            Self::Enum(x) => &x.header,
            Self::Mod(_) => return false,
        };
        header.derive_attrs.iter().any(f)
    }

    pub fn collect_fields(
        &self,
        resolve_ref: impl Fn(&RefShape) -> Vec<FieldShape>,
    ) -> Vec<FieldShape> {
        match self {
            Self::Struct(shape) => shape.fields.clone(),
            Self::AllOf(shape) => shape.expand_fields(resolve_ref),
            Self::OneOf(shape) => shape.expand_fields(resolve_ref),
            Self::NewType { .. } | Self::Enum { .. } | Self::Mod(_) => vec![],
        }
    }

    pub fn define(self) -> Result<Definition> {
        match self {
            Self::Struct(StructShape { header, fields }) => {
                let fields = fields.try_map(|field| field.define())?;
                let def = StructDef::new(header.define(), fields);
                Ok(def.into())
            }
            Self::NewType(NewTypeShape { header, type_shape }) => {
                let def = NewTypeDef::new(header.define(), type_shape.define()?);
                Ok(def.into())
            }
            Self::Enum(x) => x.define().map(|x| x.into()),
            Self::Mod(x) => x.define().map(|x| x.into()),
            Self::AllOf(_) | Self::OneOf(_) => Err(broken!(self)),
        }
    }
}

impl TryFrom<DefinitionShape> for Definition {
    type Error = gesha_core::conversions::Error;

    fn try_from(this: DefinitionShape) -> Result<Self> {
        this.define()
    }
}
