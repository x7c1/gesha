use super::to_struct::reference_to_data_type;
use crate::conversions::Error::UnknownFormat;
use crate::conversions::Result;
use crate::targets::rust_type::DataType;
use openapi_types::v3_0::SchemaCase::{Reference, Schema};
use openapi_types::v3_0::{ArrayItems, FormatModifier, OpenApiDataType, SchemaCase};

/// OpenApiDataType -> DataType
pub(super) struct TypeFactory {
    pub format: Option<FormatModifier>,
    pub items: Option<ArrayItems>,
}

impl TypeFactory {
    pub fn apply(self, data_type: OpenApiDataType) -> Result<DataType> {
        use DataType as tp;
        use FormatModifier as fm;
        use OpenApiDataType as ot;

        match (&data_type, &self.format) {
            (ot::Array, _) => {
                // TODO: remove expect()
                let items = self.items.expect("todo: array must have items");
                let item_type = items_to_type(items)?;
                Ok(tp::Vec(Box::new(item_type)))
            }
            (ot::Boolean, _) => Ok(tp::Bool),
            (ot::Integer, Some(fm::Int32)) => Ok(tp::Int32),
            (ot::Integer, Some(fm::Int64) | None) => Ok(tp::Int64),
            (ot::Number, Some(fm::Float)) => Ok(tp::Float32),
            (ot::Number, Some(fm::Double) | None) => Ok(tp::Float64),
            (ot::Object, _) => unimplemented! {
                "inline object definition not implemented: {:?}",
                data_type
            },
            (ot::String, _) => Ok(tp::String),
            (_, Some(x)) => Err(UnknownFormat {
                data_type,
                format: x.to_string(),
            }),
        }
    }
}

fn items_to_type(items: ArrayItems) -> Result<DataType> {
    let case: SchemaCase = items.into();
    match case {
        Schema(object) => {
            let factory = TypeFactory {
                format: object.format,
                items: object.items,
            };
            let data_type = object.data_type.unwrap_or_else(|| unimplemented!());
            factory.apply(data_type)
        }
        Reference(object) => reference_to_data_type(object),
    }
}
