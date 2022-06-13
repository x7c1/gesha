use crate::conversions::v3_0::to_rust_type::{
    object_to_field_shapes, shape_schema_object_type, DefinitionShape, FieldShape, TypeShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::{DataType, StructDef};
use openapi_types::v3_0::{ReferenceObject, SchemaCase, SchemaFieldName, SchemaObject};
use SchemaCase::{Reference, Schema};

pub(super) fn to_struct(name: SchemaFieldName, object: SchemaObject) -> Result<DefinitionShape> {
    let field_shapes = object_to_field_shapes(object)?;
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

pub(super) fn shape_type(schema_case: SchemaCase) -> Result<TypeShape> {
    match schema_case {
        Schema(object) => shape_schema_object_type(*object),
        Reference(object) => shape_schema_reference_type(object),
    }
}

fn shape_schema_reference_type(object: ReferenceObject) -> Result<TypeShape> {
    let type_name = match String::from(object) {
        x if x.starts_with("#/components/schemas/") => {
            // TODO: change location to relative paths by TypeShape::Ref
            // if using "#/components/responses/" etc
            x.replace("#/components/schemas/", "")
        }
        x => unimplemented!("not implemented: {x}"),
    };

    Ok(TypeShape::Fixed(DataType::Custom(type_name)))
}
