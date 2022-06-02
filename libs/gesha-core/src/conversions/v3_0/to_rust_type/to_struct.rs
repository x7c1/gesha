use crate::conversions::Error::{FieldTypeMissing, UnknownFormat};
use crate::conversions::Result;
use crate::targets::rust_type::{Definition, FieldType, StructDef, StructField};
use openapi_types::v3_0::{
    FormatModifier, OpenApiDataType, RequiredSchemaFields, SchemaCase, SchemaFieldName,
    SchemaObject, SchemaProperties,
};
use SchemaCase::{Reference, Schema};

pub(super) fn to_struct(name: SchemaFieldName, object: SchemaObject) -> Result<Definition> {
    let to_fields = |properties| {
        let factory = FieldsFactory {
            required: object.required,
        };
        factory.apply(properties)
    };
    let def = StructDef {
        name: name.into(),
        fields: object.properties.map(to_fields).unwrap_or(Ok(vec![]))?,
    };
    Ok(def.into())
}

/// SchemaProperties -> Vec<StructField>
struct FieldsFactory {
    required: Option<RequiredSchemaFields>,
}

impl FieldsFactory {
    fn apply(self, props: SchemaProperties) -> Result<Vec<StructField>> {
        props.into_iter().map(|x| self.to_field(x)).collect()
    }

    fn to_field(&self, entry: (SchemaFieldName, SchemaCase)) -> Result<StructField> {
        let (field_name, schema_case) = entry;
        match schema_case {
            Schema(object) => {
                let to_fields = |data_type| {
                    let factory = FieldFactory::new(field_name, object.format, &self.required);
                    factory.apply(data_type)
                };
                object
                    .data_type
                    .map(to_fields)
                    .unwrap_or(Err(FieldTypeMissing))
            }

            // TODO:
            Reference(reference_object) => {
                unimplemented!("reference field not implemented: {:?}", reference_object)
            }
        }
    }
}

/// OpenApiDataType -> StructField
struct FieldFactory {
    field_name: SchemaFieldName,
    format: Option<FormatModifier>,
    is_required: bool,
}

impl FieldFactory {
    fn new(
        field_name: SchemaFieldName,
        format: Option<FormatModifier>,
        required: &Option<RequiredSchemaFields>,
    ) -> FieldFactory {
        let is_required = match &required {
            Some(required) => required.contains(field_name.as_ref()),
            None => false,
        };
        Self {
            field_name,
            format,
            is_required,
        }
    }

    fn apply(self, openapi_type: OpenApiDataType) -> Result<StructField> {
        let mut data_type = self.to_field_type(openapi_type)?;
        if !self.is_required {
            data_type = FieldType::Option(Box::new(data_type))
        }
        Ok(StructField {
            name: self.field_name.into(),
            data_type,
        })
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
