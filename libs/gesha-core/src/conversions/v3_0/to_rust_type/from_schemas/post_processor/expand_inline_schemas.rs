use crate::conversions::v3_0::to_rust_type::from_schemas::to_shape::to_shape;
use crate::conversions::v3_0::to_rust_type::from_schemas::DefinitionShape::Mod;
use crate::conversions::v3_0::to_rust_type::from_schemas::{
    DefinitionShape, FieldShape, PostProcessor, StructShape, TypeShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::DataType;
use openapi_types::v3_0::ComponentName;
use TypeShape::{Array, Fixed, InlineObject, Ref};

impl PostProcessor {
    pub fn process_inline_schemas(&self, shapes: &mut Vec<DefinitionShape>) -> Result<()> {
        let mut expanded_mod_shapes = shapes
            .iter_mut()
            .filter_map(|x| x.as_mut_struct())
            .map(expand_struct_fields)
            .collect::<Result<Vec<Option<_>>>>()?
            .into_iter()
            .flatten()
            .collect();

        shapes.append(&mut expanded_mod_shapes);
        Ok(())
    }
}

fn expand_struct_fields(shape: &mut StructShape) -> Result<Option<DefinitionShape>> {
    let mod_name = shape.header.name.to_snake_case();
    let expanded_shapes = shape
        .fields
        .iter_mut()
        .map(|field| expand(&mod_name, field))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    if expanded_shapes.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Mod {
            name: mod_name,
            defs: expanded_shapes,
        }))
    }
}

fn expand(mod_name: &ComponentName, field: &mut FieldShape) -> Result<Vec<DefinitionShape>> {
    match &field.type_shape {
        Ref { .. } | Fixed { .. } | Array { .. } => Ok(vec![]),
        InlineObject {
            object,
            is_required,
            is_nullable,
        } => {
            let struct_name = field.name.to_upper_camel_case();

            // TODO: support nested modules like foo::bar::Baz
            let data_type = DataType::Custom(format!("{}::{}", mod_name, struct_name));
            let generated_struct = to_shape((struct_name, object.clone().into()))?;

            field.type_shape = Fixed {
                data_type,
                is_required: *is_required,
                is_nullable: *is_nullable,
            };

            // TODO: generate DefinitionShape from generated_struct
            Ok(vec![generated_struct])
        }
    }
}
