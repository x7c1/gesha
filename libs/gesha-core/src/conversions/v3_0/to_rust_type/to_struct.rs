use crate::conversions::v3_0::to_rust_type::PostProcess::Struct;
use crate::conversions::v3_0::to_rust_type::{object_to_field_shapes, DefinitionShape, FieldShape};
use crate::conversions::Result;
use crate::targets::rust_type::{StructDef, StructField};
use openapi_types::v3_0::{SchemaFieldName, SchemaObject};

pub(super) fn to_struct(name: SchemaFieldName, object: SchemaObject) -> Result<DefinitionShape> {
    let field_shapes = object_to_field_shapes(object)?;
    let fields = field_shapes
        .iter()
        .filter_map(|x| match x {
            FieldShape::Fixed(field) => Some(field.clone()),
            FieldShape::InProcess { .. } => None,
        })
        .collect::<Vec<StructField>>();

    let shape = if fields.len() == field_shapes.len() {
        let def = StructDef {
            name: name.into(),
            fields,
        };
        DefinitionShape::Fixed(def.into())
    } else {
        let process = Struct {
            struct_name: name.into(),
            shapes: field_shapes,
        };
        process.into()
    };
    Ok(shape)
}
