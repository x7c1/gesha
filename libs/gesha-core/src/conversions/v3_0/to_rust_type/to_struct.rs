use crate::conversions::Error::FieldTypeMissing;
use crate::conversions::Result;
use crate::targets::rust_type::{Definition, FieldType, StructDef, StructField};
use openapi_types::v3_0::{
    OpenApiDataType, RequiredSchemaFields, SchemaCase, SchemaFieldName, SchemaObject,
    SchemaProperties,
};

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
            SchemaCase::Schema(schema_object) => match schema_object.data_type {
                Some(openapi_type) => {
                    let mut data_type = to_field_type(openapi_type)?;
                    let name: String = field_name.into();

                    if !self.is_required(&name) {
                        data_type = FieldType::Option(Box::new(data_type))
                    }
                    Ok(StructField { name, data_type })
                }
                None => Err(FieldTypeMissing),
            },
            // TODO:
            SchemaCase::Reference(reference_object) => {
                unimplemented!("reference field not implemented: {:?}", reference_object)
            }
        }
    }

    fn is_required(&self, name: &str) -> bool {
        match &self.required {
            Some(required) => required.contains(name),
            None => false,
        }
    }
}

fn to_field_type(data_type: OpenApiDataType) -> Result<FieldType> {
    match data_type {
        OpenApiDataType::String => Ok(FieldType::String),
        // TODO: receive "format"
        OpenApiDataType::Integer => Ok(FieldType::Int64),
        // TODO: receive "items"
        OpenApiDataType::Array => Ok(FieldType::Vec),
        OpenApiDataType::Object => {
            unimplemented!("inline object definition not implemented: {:?}", data_type)
        }
    }
}
