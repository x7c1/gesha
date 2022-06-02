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
            Schema(object) => object
                .data_type
                .map(self.on_schema(field_name, object.format))
                .unwrap_or(Err(FieldTypeMissing)),

            // TODO:
            Reference(reference_object) => {
                unimplemented!("reference field not implemented: {:?}", reference_object)
            }
        }
    }

    fn on_schema(
        &self,
        field_name: SchemaFieldName,
        format: Option<FormatModifier>,
    ) -> impl FnOnce(OpenApiDataType) -> Result<StructField> + '_ {
        |openapi_type| {
            let mut data_type = self.to_field_type(openapi_type, format)?;
            let name: String = field_name.into();
            if !self.is_required(&name) {
                data_type = FieldType::Option(Box::new(data_type))
            }
            Ok(StructField { name, data_type })
        }
    }

    fn is_required(&self, name: &str) -> bool {
        match &self.required {
            Some(required) => required.contains(name),
            None => false,
        }
    }

    fn to_field_type(
        &self,
        data_type: OpenApiDataType,
        format: Option<FormatModifier>,
    ) -> Result<FieldType> {
        match (&data_type, format) {
            (OpenApiDataType::String, _) => Ok(FieldType::String),
            (OpenApiDataType::Integer, Some(FormatModifier::Int32)) => Ok(FieldType::Int32),
            (OpenApiDataType::Integer, Some(FormatModifier::Int64) | None) => Ok(FieldType::Int64),
            (OpenApiDataType::Number, Some(FormatModifier::Float)) => Ok(FieldType::Float32),
            (OpenApiDataType::Number, Some(FormatModifier::Double) | None) => {
                Ok(FieldType::Float64)
            }
            // TODO: receive "items"
            (OpenApiDataType::Array, _) => Ok(FieldType::Vec),
            (OpenApiDataType::Object, _) => {
                unimplemented!("inline object definition not implemented: {:?}", data_type)
            }
            (_, Some(x)) => Err(UnknownFormat {
                data_type,
                format: x.to_string(),
            }),
        }
    }
}
