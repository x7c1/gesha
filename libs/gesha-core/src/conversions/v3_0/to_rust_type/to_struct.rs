use super::type_factory::TypeFactory;
use crate::conversions::Result;
use crate::targets::rust_type::{DataType, Definition, StructDef, StructField, StructFieldName};
use openapi_types::v3_0::{
    OpenApiDataType, ReferenceObject, RequiredSchemaFields, SchemaCase, SchemaFieldName,
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
            Schema(object) => self.schema_to_field(field_name, object),
            Reference(object) => self.reference_to_field(field_name, object),
        }
    }

    fn schema_to_field(&self, name: SchemaFieldName, object: SchemaObject) -> Result<StructField> {
        match object.data_type {
            Some(data_type) => {
                let factory = FieldFactory {
                    required: &self.required,
                    to_type: TypeFactory {
                        format: object.format,
                        items: object.items,
                    },
                };
                factory.apply(name, data_type)
            }
            None => unimplemented!(),
        }
    }

    fn reference_to_field(
        &self,
        name: SchemaFieldName,
        object: ReferenceObject,
    ) -> Result<StructField> {
        Ok(StructField {
            name: StructFieldName::new(name),
            data_type: reference_to_data_type(object)?,
        })
    }
}

pub(super) fn reference_to_data_type(object: ReferenceObject) -> Result<DataType> {
    let type_name = match String::from(object) {
        x if x.starts_with("#/components/schemas/") => {
            // TODO: change location to relative paths if using "#/components/responses/" etc
            x.replace("#/components/schemas/", "")
        }
        x => unimplemented!("not implemented: {x}"),
    };
    Ok(DataType::Custom(type_name))
}

/// (SchemaFieldName, OpenApiDataType) -> StructField
struct FieldFactory<'a> {
    required: &'a Option<RequiredSchemaFields>,
    to_type: TypeFactory,
}

impl<'a> FieldFactory<'a> {
    fn apply(self, name: SchemaFieldName, data_type: OpenApiDataType) -> Result<StructField> {
        let is_required = self.is_required(&name);
        let mut field_type = self.to_type.apply(data_type)?;
        if !is_required {
            field_type = DataType::Option(Box::new(field_type))
        }
        Ok(StructField {
            name: StructFieldName::new(name),
            data_type: field_type,
        })
    }

    fn is_required(&self, field_name: &SchemaFieldName) -> bool {
        match self.required {
            Some(required) => required.contains(field_name.as_ref()),
            None => false,
        }
    }
}
