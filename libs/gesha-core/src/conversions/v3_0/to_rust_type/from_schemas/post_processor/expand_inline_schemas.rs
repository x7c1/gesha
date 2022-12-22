use crate::conversions::v3_0::to_rust_type::from_schemas::{
    DefinitionShape, FieldShape, PostProcessor, StructShape, TypeHeaderShape, TypeShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::DataType;
use TypeShape::{Array, Fixed, InlineObject, Ref};

impl PostProcessor {
    pub fn expand_inline_schemas(&self, shapes: &mut Vec<DefinitionShape>) -> Result<()> {
        let mut inline_shapes = shapes
            .iter_mut()
            .filter_map(|x| x.as_struct_shape())
            .flat_map(modify_struct_shape)
            .collect::<Result<Vec<_>>>()?
            .concat();

        shapes.append(&mut inline_shapes);
        Ok(())
    }
}

fn modify_struct_shape(
    shape: &mut StructShape,
) -> impl Iterator<Item = Result<Vec<DefinitionShape>>> + '_ {
    shape
        .fields
        .iter_mut()
        .map(|x| to_mod_defs(&shape.header, x))
}

fn to_mod_defs(header: &TypeHeaderShape, field: &mut FieldShape) -> Result<Vec<DefinitionShape>> {
    match &field.type_shape {
        Ref { .. } | Fixed { .. } | Array { .. } => Ok(vec![]),
        InlineObject {
            object,
            is_required,
            is_nullable,
        } => {
            println!("target inline object: {:#?}", object);
            let parent = &header.name;
            println!("parent: {:#?}", parent);

            field.type_shape = Fixed {
                // TODO: generate type name like pet::RegisteredProfile
                data_type: DataType::Custom("TODO".to_string()),
                is_required: *is_required,
                is_nullable: *is_nullable,
            };
            // TODO: generate DefinitionShape::Mod from object and return it
            Ok(vec![])
        }
    }
}
