use crate::conversions::v3_0::to_rust_type::{shape_type, TypeShape};
use crate::conversions::Error::UnknownFormat;
use crate::conversions::Result;
use crate::targets::rust_type::DataType;
use openapi_types::v3_0::{ArrayItems, FormatModifier, OpenApiDataType, SchemaObject};
use TypeShape::Fixed;

pub fn shape_schema_object_type(object: SchemaObject) -> Result<TypeShape> {
    match object.data_type {
        Some(data_type) => {
            let to_type = TypeFactory {
                format: object.format,
                items: object.items,
            };
            to_type.apply(data_type)
        }
        None => unimplemented!(),
    }
}

/// OpenApiDataType -> TypeShape
struct TypeFactory {
    pub format: Option<FormatModifier>,
    pub items: Option<ArrayItems>,
}

impl TypeFactory {
    pub fn apply(self, data_type: OpenApiDataType) -> Result<TypeShape> {
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

        let item_type = shape_type(items.into())?;
        match item_type {
            Fixed(data_type) => Ok(Fixed(DataType::Vec(Box::new(data_type)))),
            _ => Ok(TypeShape::Vec(Box::new(item_type))),
        }
    }
}
