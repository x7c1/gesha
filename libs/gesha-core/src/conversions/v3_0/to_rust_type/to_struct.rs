use super::type_factory::TypeFactory;
use crate::conversions::v3_0::to_rust_type::Fragment::Fixed;
use crate::conversions::v3_0::to_rust_type::{Fragment, FragmentStructField, FragmentType};
use crate::conversions::Result;
use crate::targets::rust_type::{DataType, StructDef, StructField, StructFieldName};
use openapi_types::v3_0::{
    ReferenceObject, RequiredSchemaFields, SchemaCase, SchemaFieldName, SchemaObject,
    SchemaProperties,
};
use SchemaCase::{Reference, Schema};

pub(super) fn to_struct(name: SchemaFieldName, object: SchemaObject) -> Result<Fragment> {
    let to_fields = |properties| {
        let factory = FieldsFactory {
            required: object.required,
        };
        factory.apply(properties)
    };
    let fragments = object.properties.map(to_fields).unwrap_or(Ok(vec![]))?;
    let in_process = fragments
        .iter()
        .any(|x| matches!(x, FragmentStructField::InProcess { .. }));

    let fragment = if in_process {
        unimplemented!()
    } else {
        let fields = fragments
            .into_iter()
            .map(|x| match x {
                FragmentStructField::Fixed(field) => field,
                FragmentStructField::InProcess { .. } => unimplemented!(),
            })
            .collect();

        let def = StructDef {
            name: name.into(),
            fields,
        };
        Fixed(def.into())
    };
    Ok(fragment)
}

/// SchemaProperties -> Vec<StructField>
struct FieldsFactory {
    required: Option<RequiredSchemaFields>,
}

impl FieldsFactory {
    fn apply(self, props: SchemaProperties) -> Result<Vec<FragmentStructField>> {
        props
            .into_iter()
            .map(|(name, case)| self.to_field(name, case))
            .collect()
    }

    fn to_field(&self, name: SchemaFieldName, case: SchemaCase) -> Result<FragmentStructField> {
        match to_data_type(case)? {
            FragmentType::Fixed(mut data_type) => {
                if !self.is_required(&name) {
                    data_type = DataType::Option(Box::new(data_type));
                }
                Ok(FragmentStructField::Fixed(StructField {
                    name: StructFieldName::new(name),
                    data_type,
                }))
            }
            fragment_type => Ok(FragmentStructField::InProcess {
                name: StructFieldName::new(name),
                data_type: fragment_type,
            }),
        }
    }

    fn is_required(&self, name: &SchemaFieldName) -> bool {
        match &self.required {
            Some(required) => required.contains(name.as_ref()),
            None => false,
        }
    }
}

pub(super) fn to_data_type(schema_case: SchemaCase) -> Result<FragmentType> {
    match schema_case {
        Schema(object) => schema_object_to_data_type(*object),
        Reference(object) => schema_ref_to_data_type(object),
    }
}

pub(super) fn schema_object_to_data_type(object: SchemaObject) -> Result<FragmentType> {
    match object.data_type {
        Some(data_type) => {
            let to_type = TypeFactory {
                format: object.format,
                items: object.items,
            };
            to_type.apply(data_type)
        }
        None => unimplemented!(),
    }
}

fn schema_ref_to_data_type(object: ReferenceObject) -> Result<FragmentType> {
    let type_name = match String::from(object) {
        x if x.starts_with("#/components/schemas/") => {
            // TODO: change location to relative paths if using "#/components/responses/" etc
            // TODO: use FragmentType::Ref
            x.replace("#/components/schemas/", "")
        }
        x => unimplemented!("not implemented: {x}"),
    };

    Ok(FragmentType::Fixed(DataType::Custom(type_name)))
}
