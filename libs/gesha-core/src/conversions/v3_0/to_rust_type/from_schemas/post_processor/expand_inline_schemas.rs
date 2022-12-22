use crate::conversions::v3_0::to_rust_type::from_schemas::{
    DefinitionShape, PostProcessor, StructShape, TypeShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::DataType;

impl PostProcessor {
    pub fn expand_inline_schemas(&self, shapes: &mut Vec<DefinitionShape>) -> Result<()> {
        let mut inline_shapes = shapes
            .iter_mut()
            .filter_map(|x| x.as_struct_shape())
            .map(modify_struct_shape)
            .collect::<Result<Vec<Vec<DefinitionShape>>>>()?
            .into_iter()
            .flatten()
            .collect();

        shapes.append(&mut inline_shapes);
        Ok(())
    }
}

fn modify_struct_shape(shape: &mut StructShape) -> Result<Vec<DefinitionShape>> {
    shape.shapes.iter_mut().for_each(|x| {
        match &x.type_shape {
            TypeShape::Ref { .. } | TypeShape::Fixed { .. } | TypeShape::Vec { .. } => {}
            TypeShape::InlineObject {
                object,
                is_required,
                is_nullable,
            } => {
                // TODO: generate DefinitionShape from object and push it to original Vec<DefinitionShape>
                println!("target inline object: {:#?}", object);
                x.type_shape = TypeShape::Fixed {
                    // TODO: generate type name like pet::RegisteredProfile
                    data_type: DataType::Custom("TODO".to_string()),
                    is_required: *is_required,
                    is_nullable: *is_nullable,
                }
            }
        }
    });

    // TODO:
    Ok(vec![])
}
