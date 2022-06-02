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
            Schema(object) => self.translate(field_name, object),

            // TODO:
            Reference(reference_object) => {
                unimplemented!("reference field not implemented: {:?}", reference_object)
            }
        }
    }

    fn translate(&self, name: SchemaFieldName, object: SchemaObject) -> Result<StructField> {
        match object.data_type {
            Some(data_type) => {
                let factory = FieldFactory {
                    format: object.format,
                    required: &self.required,
                };
                factory.apply(name, data_type)
            }
            None => Err(FieldTypeMissing),
        }
    }
}

/// (SchemaFieldName, OpenApiDataType) -> StructField
struct FieldFactory<'a> {
    format: Option<FormatModifier>,
    required: &'a Option<RequiredSchemaFields>,
}

impl<'a> FieldFactory<'a> {
    fn apply(self, name: SchemaFieldName, data_type: OpenApiDataType) -> Result<StructField> {
        let mut field_type = self.to_field_type(data_type)?;
        if !self.is_required(&name) {
            field_type = FieldType::Option(Box::new(field_type))
        }
        Ok(StructField {
            name: name.into(),
            data_type: field_type,
        })
    }

    fn is_required(&self, field_name: &SchemaFieldName) -> bool {
        match self.required {
            Some(required) => required.contains(field_name.as_ref()),
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
