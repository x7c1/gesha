use crate::conversions::v3_0::to_rust_type::from_schemas::{
    DefinitionShape, PostProcessor, TypeShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::DataType;

impl PostProcessor {
    pub fn expand_inline_schemas(&self, shapes: &mut Vec<DefinitionShape>) -> Result<()> {
        let mut inline_shapes = shapes
            .iter_mut()
            .filter(|x| x.has_inline_schemas())
            .map(modify_inline_type)
            .collect::<Result<Vec<Vec<DefinitionShape>>>>()?
            .into_iter()
            .flatten()
            .collect();

        shapes.append(&mut inline_shapes);
        Ok(())
    }
}

fn modify_inline_type(shape: &mut DefinitionShape) -> Result<Vec<DefinitionShape>> {
    let parent_type_name = shape.type_name().clone();
    match shape {
        DefinitionShape::Struct { shapes, .. } => {
            println!("parent_name: {}", parent_type_name);

            shapes.iter_mut().for_each(|x| {
                match &x.type_shape {
                    TypeShape::Fixed { .. } => {}
                    TypeShape::Vec { .. } => {}
                    TypeShape::Ref { .. } => {}
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
            // TODO
            Ok(vec![])
        }
        DefinitionShape::AllOf { .. }
        | DefinitionShape::NewType { .. }
        | DefinitionShape::Enum { .. } => Ok(vec![]),
    }
}
