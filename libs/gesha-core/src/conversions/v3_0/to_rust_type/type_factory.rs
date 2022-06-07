use super::to_struct::to_data_type;
use crate::conversions::Error::UnknownFormat;
use crate::conversions::Result;
use crate::targets::rust_type::DataType;
use openapi_types::v3_0::{ArrayItems, FormatModifier, OpenApiDataType};

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
                let items = self
                    .items
                    .unwrap_or_else(|| unimplemented!("array must have items"));

                let item_type = to_data_type(items.into())?;
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
