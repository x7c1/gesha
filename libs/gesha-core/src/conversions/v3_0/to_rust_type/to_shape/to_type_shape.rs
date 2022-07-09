use crate::conversions::v3_0::to_rust_type::TypeShape;
use crate::conversions::Error::UnknownFormat;
use crate::conversions::Result;
use crate::targets::rust_type::DataType;
use openapi_types::v3_0::SchemaCase;
use openapi_types::v3_0::SchemaCase::{Reference, Schema};
use openapi_types::v3_0::{ArrayItems, FormatModifier, OpenApiDataType, SchemaObject};
use TypeShape::Fixed;

pub(super) fn to_type_shape(schema_case: SchemaCase, is_required: bool) -> Result<TypeShape> {
    let mut shape = match schema_case {
        Schema(object) => from_object(*object),
        Reference(object) => Ok(TypeShape::Ref(object)),
    }?;
    if !is_required {
        shape = TypeShape::Option(Box::new(shape));
    }
    Ok(shape)
}

pub(super) fn from_object(object: SchemaObject) -> Result<TypeShape> {
    match object.data_type {
        Some(data_type) => {
            let to_type = TypeFactory {
                format: object.format,
                items: object.items,
                nullable: object.nullable,
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
}

impl TypeFactory {
    fn apply(self, data_type: OpenApiDataType) -> Result<TypeShape> {
        use DataType as tp;
        use FormatModifier as fm;
        use OpenApiDataType as ot;

        match (&data_type, &self.format) {
            (ot::Array, _) => self.items_to_shape(),
            (ot::Boolean, _) => Ok(Fixed(tp::Bool)),
            (ot::Integer, Some(fm::Int32)) => Ok(Fixed(tp::Int32)),
            (ot::Integer, Some(fm::Int64) | None) => Ok(Fixed(tp::Int64)),
            (ot::Number, Some(fm::Float)) => Ok(Fixed(tp::Float32)),
            (ot::Number, Some(fm::Double) | None) => Ok(Fixed(tp::Float64)),
            (ot::String, _) => Ok(Fixed(tp::String)),
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

        let items_shape = match SchemaCase::from(items) {
            Schema(object) => from_object(*object),
            Reference(object) => Ok(TypeShape::Ref(object)),
        }?;
        let type_shape = match items_shape {
            Fixed(data_type) => Fixed(DataType::Vec(Box::new(data_type))),
            shape => TypeShape::Vec(Box::new(shape)),
        };
        Ok(type_shape)
    }
}
