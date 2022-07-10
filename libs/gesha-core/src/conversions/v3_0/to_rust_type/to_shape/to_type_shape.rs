use crate::conversions::v3_0::to_rust_type::TypeShape;
use crate::conversions::Error::UnknownFormat;
use crate::conversions::Result;
use crate::targets::rust_type::DataType;
use openapi_types::v3_0::SchemaCase;
use openapi_types::v3_0::SchemaCase::{Reference, Schema};
use openapi_types::v3_0::{ArrayItems, FormatModifier, OpenApiDataType, SchemaObject};
use TypeShape::Fixed;

pub(super) fn to_type_shape(schema_case: SchemaCase, is_required: bool) -> Result<TypeShape> {
    let shape = match schema_case {
        Schema(object) => from_object(*object, is_required)?,
        Reference(object) => TypeShape::Ref {
            object,
            is_required,
        },
    };
    Ok(shape)
}

pub(super) fn from_object(object: SchemaObject, is_required: bool) -> Result<TypeShape> {
    match object.data_type {
        Some(data_type) => {
            let to_type = TypeFactory {
                format: object.format,
                items: object.items,
                nullable: object.nullable,
                is_required,
            };
            to_type.apply(data_type)
        }
        None => unimplemented!(),
    }
}

/// OpenApiDataType -> TypeShape
struct TypeFactory {
    format: Option<FormatModifier>,
    items: Option<ArrayItems>,
    // TODO:
    #[allow(dead_code)]
    nullable: Option<bool>,
    is_required: bool,
}

impl TypeFactory {
    fn apply(self, data_type: OpenApiDataType) -> Result<TypeShape> {
        use DataType as tp;
        use FormatModifier as fm;
        use OpenApiDataType as ot;

        let is_required = self.is_required;
        let is_nullable = self.nullable.unwrap_or(false);
        match (&data_type, &self.format) {
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
            (ot::Object, _) => unimplemented! {
                "inline object definition not implemented: {:?}",
                data_type
            },
            (_, Some(x)) => Err(UnknownFormat {
                data_type,
                format: x.to_string(),
            }),
        }
    }

    fn items_to_shape(self) -> Result<TypeShape> {
        let items = self
            .items
            .unwrap_or_else(|| unimplemented!("array must have items"));

        let items_shape = to_type_shape(items.into(), /* is_required */ true)?;
        let type_shape = TypeShape::Vec {
            type_shape: Box::new(items_shape),
            is_required: self.is_required,
        };
        Ok(type_shape)
    }
}
