use crate::conversions::Error::UnknownFormat;
use crate::conversions::Result;
use crate::targets::rust_type::{DataType, Definition, StructDef, StructField};
use openapi_types::v3_0::{
    ArrayItems, FormatModifier, OpenApiDataType, RequiredSchemaFields, SchemaCase, SchemaFieldName,
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
            Reference(reference_object) => {
                // TODO:
                unimplemented!("reference field not implemented: {:?}", reference_object)
            }
        }
    }

    fn translate(&self, name: SchemaFieldName, object: SchemaObject) -> Result<StructField> {
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
}

/// OpenApiDataType -> DataType
struct TypeFactory {
    format: Option<FormatModifier>,
    items: Option<ArrayItems>,
}

impl TypeFactory {
    fn apply(self, data_type: OpenApiDataType) -> Result<DataType> {
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
            // TODO: remove unwrap()
            let data_type = object.data_type.unwrap();
            factory.apply(data_type)
        }
        Reference(_) => unimplemented!(),
    }
}
