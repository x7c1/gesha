use crate::conversions::v3_0::to_rust_type::PostProcess::Struct;
use crate::conversions::v3_0::to_rust_type::{object_to_field_shapes, DefinitionShape, FieldShape};
use crate::conversions::Result;
use crate::targets::rust_type::StructDef;
use openapi_types::v3_0::{SchemaFieldName, SchemaObject};

pub(super) fn to_struct(name: SchemaFieldName, object: SchemaObject) -> Result<DefinitionShape> {
    let field_shapes = object_to_field_shapes(object)?;
    let in_process = field_shapes
        .iter()
        .any(|x| matches!(x, FieldShape::InProcess { .. }));

    let shape = if in_process {
        DefinitionShape::InProcess(Struct {
            struct_name: name.into(),
            shapes: field_shapes,
        })
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
