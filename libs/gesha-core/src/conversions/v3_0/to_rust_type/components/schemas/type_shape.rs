use crate::conversions::v3_0::to_rust_type::components::schemas::TypeShape::{Fixed, InlineObject};
use crate::conversions::v3_0::to_rust_type::components::schemas::{Optionality, TypePath};
use crate::conversions::Error::{PostProcessBroken, UnknownFormat};
use crate::conversions::Result;
use crate::targets::rust_type::DataType;
use openapi_types::v3_0::SchemaCase;
use openapi_types::v3_0::SchemaCase::{Reference, Schema};
use openapi_types::v3_0::{FormatModifier, OpenApiDataType, ReferenceObject, SchemaObject};

#[derive(Clone, Debug)]
pub enum TypeShape {
    Fixed {
        data_type: DataType,
        optionality: Optionality,
    },
    Array {
        type_shape: Box<TypeShape>,
        optionality: Optionality,
    },
    Ref {
        object: ReferenceObject<SchemaObject>,
        is_required: bool,
    },
    Expanded {
        type_path: TypePath,
        optionality: Optionality,
    },
    InlineObject {
        object: SchemaObject,
        optionality: Optionality,
    },
    Option(Box<TypeShape>),
    Patch(Box<TypeShape>),
}

impl TypeShape {
    pub fn from_case(schema_case: SchemaCase, is_required: bool) -> Result<TypeShape> {
        let shape = match schema_case {
            Schema(object) => Self::from_object(*object, is_required)?,
            Reference(object) => TypeShape::Ref {
                object,
                is_required,
            },
        };
        Ok(shape)
    }

    pub fn from_object(object: SchemaObject, is_required: bool) -> Result<TypeShape> {
        let data_type = object
            .data_type
            .clone()
            .or_else(|| object.all_of.is_some().then_some(OpenApiDataType::Object))
            .unwrap_or_else(|| unimplemented!());

        let to_type = TypeFactory {
            object,
            is_required,
        };
        to_type.apply(data_type)
    }

    pub fn resolve_optionality(self) -> Self {
        let optionality = match &self {
            Self::Fixed { optionality, .. } => optionality,
            Self::Array { optionality, .. } => optionality,
            Self::Expanded { optionality, .. } => optionality,
            Self::InlineObject { optionality, .. } => optionality,
            Self::Option(_) | Self::Patch(_) => return self,
            Self::Ref { .. } => {
                todo!()
            }
        };
        match (optionality.is_required, optionality.is_nullable) {
            (true, true) | (false, false) => TypeShape::Option(Box::new(self)),
            (false, true) => TypeShape::Patch(Box::new(self)),
            (true, false) => self,
        }
    }

    pub fn define(self) -> Result<DataType> {
        let data_type = match self {
            Self::Fixed { data_type, .. } => data_type,
            Self::Array { type_shape, .. } => DataType::Vec(Box::new((*type_shape).define()?)),
            Self::Expanded { type_path, .. } => type_path.into(),
            Self::Option(type_shape) => DataType::Option(Box::new((*type_shape).define()?)),
            Self::Patch(type_shape) => DataType::Patch(Box::new((*type_shape).define()?)),
            Self::Ref { .. } => {
                todo!()
            }
            Self::InlineObject { .. } => Err(PostProcessBroken {
                detail: format!("InlineObject must be processed before '$ref'.\n{:#?}", self),
            })?,
        };
        Ok(data_type)
    }
}

/// OpenApiDataType -> TypeShape
struct TypeFactory {
    object: SchemaObject,
    is_required: bool,
}

impl TypeFactory {
    fn apply(self, data_type: OpenApiDataType) -> Result<TypeShape> {
        use DataType as tp;
        use FormatModifier as fm;
        use OpenApiDataType as ot;

        let optionality = Optionality {
            is_required: self.is_required,
            is_nullable: self.object.nullable.unwrap_or(false),
        };
        match (&data_type, &self.object.format) {
            (ot::Array, _) => self.items_to_shape(),
            (ot::Boolean, _) => Ok(Fixed {
                data_type: tp::Bool,
                optionality,
            }),
            (ot::Integer, Some(fm::Int32)) => Ok(Fixed {
                data_type: tp::Int32,
                optionality,
            }),
            (ot::Integer, Some(fm::Int64) | None) => Ok(Fixed {
                data_type: tp::Int64,
                optionality,
            }),
            (ot::Number, Some(fm::Float)) => Ok(Fixed {
                data_type: tp::Float32,
                optionality,
            }),
            (ot::Number, Some(fm::Double) | None) => Ok(Fixed {
                data_type: tp::Float64,
                optionality,
            }),
            (ot::String, _) => Ok(Fixed {
                data_type: tp::String,
                optionality,
            }),
            (ot::Object, _) => Ok(InlineObject {
                object: self.object,
                optionality,
            }),
            (_, Some(x)) => Err(UnknownFormat {
                data_type,
                format: x.to_string(),
            }),
        }
    }

    fn items_to_shape(self) -> Result<TypeShape> {
        let items = self
            .object
            .items
            .unwrap_or_else(|| unimplemented!("array must have items"));

        let items_shape = TypeShape::from_case(items.into(), /* is_required */ true)?;
        let type_shape = TypeShape::Array {
            type_shape: Box::new(items_shape),
            optionality: Optionality {
                is_required: self.is_required,
                is_nullable: self.object.nullable.unwrap_or(false),
            },
        };
        Ok(type_shape)
    }
}
