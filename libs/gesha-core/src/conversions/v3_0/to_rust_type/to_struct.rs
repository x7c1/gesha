use crate::conversions::Error::{FieldTypeMissing, UnknownFormat};
use crate::conversions::Result;
use crate::targets::rust_type::{Definition, FieldType, StructDef, StructField};
use openapi_types::v3_0::{
    FormatModifier, OpenApiDataType, RequiredSchemaFields, SchemaCase, SchemaFieldName,
    SchemaObject, SchemaProperties,
};
use SchemaCase::{Reference, Schema};

pub(super) fn to_struct(name: SchemaFieldName, object: SchemaObject) -> Result<Definition> {
    let to_fields = ToFields {
        required: object.required,
    };
    let fields = object
        .properties
        .map(|x| to_fields.apply(x))
        .unwrap_or(Ok(vec![]))?;

    let def = StructDef {
        name: name.into(),
        fields,
    };
    Ok(def.into())
}

struct ToFields {
    required: Option<RequiredSchemaFields>,
}

impl ToFields {
    fn apply(self, props: SchemaProperties) -> Result<Vec<StructField>> {
        props.into_iter().map(|x| self.to_field(x)).collect()
    }

    fn to_field(&self, entry: (SchemaFieldName, SchemaCase)) -> Result<StructField> {
        let (field_name, schema_case) = entry;
        match schema_case {
            Schema(object) => {
                let to_field = ToField {
                    field_name,
                    format: object.format,
                    required: &self.required,
                };
                object
                    .data_type
                    .map(|x| to_field.apply(x))
                    .unwrap_or(Err(FieldTypeMissing))
            }

            // TODO:
            Reference(reference_object) => {
                unimplemented!("reference field not implemented: {:?}", reference_object)
            }
        }
    }
}

struct ToField<'a> {
    field_name: SchemaFieldName,
    format: Option<FormatModifier>,
    required: &'a Option<RequiredSchemaFields>,
}

impl ToField<'_> {
    fn apply(self, openapi_type: OpenApiDataType) -> Result<StructField> {
        let mut data_type = self.to_field_type(openapi_type)?;
        if !self.is_required() {
            data_type = FieldType::Option(Box::new(data_type))
        }
        Ok(StructField {
            name: self.field_name.into(),
            data_type,
        })
    }

    fn is_required(&self) -> bool {
        match &self.required {
            Some(required) => required.contains(self.field_name.as_ref()),
            None => false,
        }
    }

    fn to_field_type(&self, data_type: OpenApiDataType) -> Result<FieldType> {
        use FieldType as ft;
        use FormatModifier as fm;
        use OpenApiDataType as ot;

        match (&data_type, &self.format) {
            // TODO: receive "items"
            (ot::Array, _) => Ok(ft::Vec),
            (ot::Boolean, _) => Ok(ft::Bool),
            (ot::Integer, Some(fm::Int32)) => Ok(ft::Int32),
            (ot::Integer, Some(fm::Int64) | None) => Ok(ft::Int64),
            (ot::Number, Some(fm::Float)) => Ok(ft::Float32),
            (ot::Number, Some(fm::Double) | None) => Ok(ft::Float64),
            (ot::Object, _) => unimplemented! {
                "inline object definition not implemented: {:?}",
                data_type
            },
            (ot::String, _) => Ok(ft::String),
            (_, Some(x)) => Err(UnknownFormat {
                data_type,
                format: x.to_string(),
            }),
        }
    }
}
