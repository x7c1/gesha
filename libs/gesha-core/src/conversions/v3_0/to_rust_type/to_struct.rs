use super::type_factory::TypeFactory;
use crate::conversions::v3_0::to_rust_type::{DefinitionShape, FieldShape, TypeShape};
use crate::conversions::Result;
use crate::targets::rust_type::{DataType, StructDef, StructField, StructFieldName};
use openapi_types::v3_0::{
    ReferenceObject, RequiredSchemaFields, SchemaCase, SchemaFieldName, SchemaObject,
    SchemaProperties,
};
use SchemaCase::{Reference, Schema};

pub(super) fn to_struct(name: SchemaFieldName, object: SchemaObject) -> Result<DefinitionShape> {
    let to_fields = |properties| {
        let factory = FieldsFactory {
            required: object.required,
        };
        factory.apply(properties)
    };
    let field_shapes = object.properties.map(to_fields).unwrap_or(Ok(vec![]))?;
    let in_process = field_shapes
        .iter()
        .any(|x| matches!(x, FieldShape::InProcess { .. }));

    let shape = if in_process {
        unimplemented!()
    } else {
        let fields = field_shapes
            .into_iter()
            .map(|x| match x {
                FieldShape::Fixed(field) => field,
                FieldShape::InProcess { .. } => unimplemented!(),
            })
            .collect();

        let def = StructDef {
            name: name.into(),
            fields,
        };
        DefinitionShape::Fixed(def.into())
    };
    Ok(shape)
}

/// SchemaProperties -> Vec<StructField>
struct FieldsFactory {
    required: Option<RequiredSchemaFields>,
}

impl FieldsFactory {
    fn apply(self, props: SchemaProperties) -> Result<Vec<FieldShape>> {
        props
            .into_iter()
            .map(|(name, case)| self.to_field(name, case))
            .collect()
    }

    fn to_field(&self, name: SchemaFieldName, case: SchemaCase) -> Result<FieldShape> {
        match shape_type(case)? {
            TypeShape::Fixed(mut data_type) => {
                if !self.is_required(&name) {
                    data_type = DataType::Option(Box::new(data_type));
                }
                Ok(FieldShape::Fixed(StructField {
                    name: StructFieldName::new(name),
                    data_type,
                }))
            }
            type_shape => Ok(FieldShape::InProcess {
                name: StructFieldName::new(name),
                type_shape,
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

pub(super) fn shape_type(schema_case: SchemaCase) -> Result<TypeShape> {
    match schema_case {
        Schema(object) => shape_schema_object_type(*object),
        Reference(object) => shape_schema_reference_type(object),
    }
}

pub(super) fn shape_schema_object_type(object: SchemaObject) -> Result<TypeShape> {
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

fn shape_schema_reference_type(object: ReferenceObject) -> Result<TypeShape> {
    let type_name = match String::from(object) {
        x if x.starts_with("#/components/schemas/") => {
            // TODO: change location to relative paths if using "#/components/responses/" etc
            // TODO: use ShapeType::Ref
            x.replace("#/components/schemas/", "")
        }
        x => unimplemented!("not implemented: {x}"),
    };

    Ok(TypeShape::Fixed(DataType::Custom(type_name)))
}
