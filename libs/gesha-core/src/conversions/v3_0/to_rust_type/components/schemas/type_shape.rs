use crate::conversions::v3_0::to_rust_type::components::schemas::TypePath;
use crate::conversions::v3_0::to_rust_type::components::schemas::TypeShape::{Fixed, InlineObject};
use crate::conversions::Error::UnknownFormat;
use crate::conversions::Result;
use crate::targets::rust_type::DataType;
use openapi_types::v3_0::SchemaCase;
use openapi_types::v3_0::SchemaCase::{Reference, Schema};
use openapi_types::v3_0::{FormatModifier, OpenApiDataType, ReferenceObject, SchemaObject};

#[derive(Clone, Debug)]
pub enum TypeShape {
    Fixed {
        data_type: DataType,
        is_required: bool,
        is_nullable: bool,
    },
    Array {
        type_shape: Box<TypeShape>,
        is_required: bool,
        is_nullable: bool,
    },
    Ref {
        object: ReferenceObject<SchemaObject>,
        is_required: bool,
    },
    Expanded {
        type_path: TypePath,
        is_required: bool,
        is_nullable: bool,
    },
    InlineObject {
        object: SchemaObject,
        is_required: bool,
        is_nullable: bool,
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

    pub fn is_required(&self) -> bool {
        match self {
            Self::Fixed { is_required, .. } => *is_required,
            Self::Array { is_required, .. } => *is_required,
            Self::Ref { is_required, .. } => *is_required,
            Self::InlineObject { is_required, .. } => *is_required,
            Self::Expanded { is_required, .. } => *is_required,
            Self::Option { .. } | Self::Patch { .. } => false,
        }
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

        let is_required = self.is_required;
        let is_nullable = self.object.nullable.unwrap_or(false);
        match (&data_type, &self.object.format) {
            (ot::Array, _) => self.items_to_shape(),
            (ot::Boolean, _) => Ok(Fixed {
                data_type: tp::Bool,
                is_required,
                is_nullable,
            }),
            (ot::Integer, Some(fm::Int32)) => Ok(Fixed {
                data_type: tp::Int32,
                is_required,
                is_nullable,
            }),
            (ot::Integer, Some(fm::Int64) | None) => Ok(Fixed {
                data_type: tp::Int64,
                is_required,
                is_nullable,
            }),
            (ot::Number, Some(fm::Float)) => Ok(Fixed {
                data_type: tp::Float32,
                is_required,
                is_nullable,
            }),
            (ot::Number, Some(fm::Double) | None) => Ok(Fixed {
                data_type: tp::Float64,
                is_required,
                is_nullable,
            }),
            (ot::String, _) => Ok(Fixed {
                data_type: tp::String,
                is_required,
                is_nullable,
            }),
            (ot::Object, _) => Ok(InlineObject {
                object: self.object,
                is_required,
                is_nullable,
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
            is_required: self.is_required,
            is_nullable: self.object.nullable.unwrap_or(false),
        };
        Ok(type_shape)
    }
}
