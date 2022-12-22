use crate::conversions::v3_0::to_rust_type::from_schemas::{
    DefinitionShape, FieldShape, PostProcessor, StructShape, TypeHeaderShape, TypeShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::DataType;
use TypeShape::{Array, Fixed, InlineObject, Ref};

impl PostProcessor {
    pub fn process_inline_schemas(&self, shapes: &mut Vec<DefinitionShape>) -> Result<()> {
        let mut expanded_mod_shapes = shapes
            .iter_mut()
            .filter_map(|x| x.as_struct_shape())
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
    let expanded_shapes = shape
        .fields
        .iter_mut()
        .map(|field| expand(&shape.header, field))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    if expanded_shapes.is_empty() {
        Ok(None)
    } else {
        println!("expanded: {:#?}", expanded_shapes);
        // TODO: generate DefinitionShape::Mod from object and return it
        Ok(None)
    }
}

fn expand(parent_header: &TypeHeaderShape, field: &mut FieldShape) -> Result<Vec<DefinitionShape>> {
    match &field.type_shape {
        Ref { .. } | Fixed { .. } | Array { .. } => Ok(vec![]),
        InlineObject {
            object,
            is_required,
            is_nullable,
        } => {
            println!("target inline object: {:#?}", object);

            let parent = &parent_header.name;
            println!("parent: {:#?}", parent);

            field.type_shape = Fixed {
                // TODO: generate type name like pet::RegisteredProfile
                data_type: DataType::Custom("TODO".to_string()),
                is_required: *is_required,
                is_nullable: *is_nullable,
            };
            // TODO: generate DefinitionShape::Struct from object
            // TODO: generate Vec<DefinitionShape> from object.properties

            Ok(vec![])
        }
    }
}
