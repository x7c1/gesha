use super::to_struct::to_data_type;
use crate::conversions::v3_0::to_rust_type::FragmentType;
use crate::conversions::Error::UnknownFormat;
use crate::conversions::Result;
use crate::targets::rust_type::DataType;
use openapi_types::v3_0::{ArrayItems, FormatModifier, OpenApiDataType};
use FragmentType::Fixed;

/// OpenApiDataType -> DataType
pub(super) struct TypeFactory {
    pub format: Option<FormatModifier>,
    pub items: Option<ArrayItems>,
}

impl TypeFactory {
    pub fn apply(self, data_type: OpenApiDataType) -> Result<FragmentType> {
        use DataType as tp;
        use FormatModifier as fm;
        use OpenApiDataType as ot;

        match (&data_type, &self.format) {
            (ot::Array, _) => {
                let items = self
                    .items
                    .unwrap_or_else(|| unimplemented!("array must have items"));

                let item_type = to_data_type(items.into())?;
                match item_type {
                    Fixed(data_type) => Ok(Fixed(tp::Vec(Box::new(data_type)))),
                    _ => Ok(FragmentType::Vec(Box::new(item_type))),
                }
            }
            (ot::Boolean, _) => Ok(Fixed(tp::Bool)),
            (ot::Integer, Some(fm::Int32)) => Ok(Fixed(tp::Int32)),
            (ot::Integer, Some(fm::Int64) | None) => Ok(Fixed(tp::Int64)),
            (ot::Number, Some(fm::Float)) => Ok(Fixed(tp::Float32)),
            (ot::Number, Some(fm::Double) | None) => Ok(Fixed(tp::Float64)),
            (ot::Object, _) => unimplemented! {
                "inline object definition not implemented: {:?}",
                data_type
            },
            (ot::String, _) => Ok(Fixed(tp::String)),
            (_, Some(x)) => Err(UnknownFormat {
                data_type,
                format: x.to_string(),
            }),
        }
    }
}
