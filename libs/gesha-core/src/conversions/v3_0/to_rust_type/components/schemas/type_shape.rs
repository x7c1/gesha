use crate::conversions::v3_0::to_rust_type::components::schemas::TypeShape::{Inline, Proper};
use crate::conversions::v3_0::to_rust_type::components::schemas::{Optionality, TypePath};
use crate::conversions::Error::{PostProcessBroken, UnknownFormat};
use crate::conversions::Result;
use crate::targets::rust_type::DataType;
use openapi_types::v3_0::SchemaCase;
use openapi_types::v3_0::SchemaCase::{Reference, Schema};
use openapi_types::v3_0::{FormatModifier, OpenApiDataType, ReferenceObject, SchemaObject};

#[derive(Clone, Debug)]
pub enum TypeShape {
    Proper {
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
    Inline {
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

    pub fn resolve_optionality(self) -> Result<Self> {
        let optionality = match &self {
            Self::Proper { optionality, .. }
            | Self::Array { optionality, .. }
            | Self::Expanded { optionality, .. } => optionality,
            Self::Option(_) | Self::Patch(_) => {
                // already resolved
                return Ok(self);
            }
            Self::Inline { .. } => {
                todo!()
            }
            Self::Ref { .. } => {
                todo!()
            }
        };
        let resolved = match (optionality.is_required, optionality.is_nullable) {
            (true, true) | (false, false) => TypeShape::Option(Box::new(self)),
            (false, true) => TypeShape::Patch(Box::new(self)),
            (true, false) => self,
        };
        Ok(resolved)
    }

    pub fn define(self) -> Result<DataType> {
        let data_type = match self {
            Self::Proper { data_type, .. } => data_type,
            Self::Array { type_shape, .. } => DataType::Vec(Box::new((*type_shape).define()?)),
            Self::Expanded { type_path, .. } => type_path.into(),
            Self::Option(type_shape) => DataType::Option(Box::new((*type_shape).define()?)),
            Self::Patch(type_shape) => DataType::Patch(Box::new((*type_shape).define()?)),
            Self::Ref { .. } => {
                todo!()
            }
            Self::Inline { .. } => Err(PostProcessBroken {
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
            (ot::Boolean, _) => Ok(Proper {
                data_type: tp::Bool,
                optionality,
            }),
            (ot::Integer, Some(fm::Int32)) => Ok(Proper {
                data_type: tp::Int32,
                optionality,
            }),
            (ot::Integer, Some(fm::Int64) | None) => Ok(Proper {
                data_type: tp::Int64,
                optionality,
            }),
            (ot::Number, Some(fm::Float)) => Ok(Proper {
                data_type: tp::Float32,
                optionality,
            }),
            (ot::Number, Some(fm::Double) | None) => Ok(Proper {
                data_type: tp::Float64,
                optionality,
            }),
            (ot::String, _) => Ok(Proper {
                data_type: tp::String,
                optionality,
            }),
            (ot::Object, _) => Ok(Inline {
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
