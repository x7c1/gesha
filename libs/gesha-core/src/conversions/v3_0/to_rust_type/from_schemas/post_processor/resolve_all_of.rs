use crate::conversions::v3_0::to_rust_type::from_schemas::post_processor::PostProcessor;
use crate::conversions::v3_0::to_rust_type::from_schemas::{
    AllOfItemShape, AllOfShape, DefinitionShape, FieldShape, StructShape,
};
use crate::conversions::Result;

impl PostProcessor {
    pub fn process_all_of(&self, shapes: &mut [DefinitionShape]) -> Result<()> {
        shapes.iter_mut().try_for_each(|x| self.resolve_all_of(x))
    }

    fn resolve_all_of(&self, shape: &mut DefinitionShape) -> Result<()> {
        if let Some(processed) = self.shape_all_of(shape)? {
            *shape = processed;
        }
        Ok(())
    }

    fn shape_all_of(&self, def_shape: &mut DefinitionShape) -> Result<Option<DefinitionShape>> {
        match def_shape {
            DefinitionShape::AllOf(AllOfShape { header, items }) => {
                let shape = DefinitionShape::Struct(StructShape {
                    header: header.clone(),
                    fields: self.merge_fields_all_of(items)?,
                });
                Ok(Some(shape))
            }
            DefinitionShape::Mod { defs, .. } => {
                self.process_all_of(defs)?;
                Ok(None)
            }
            // shaped in other processes.
            DefinitionShape::Struct { .. }
            | DefinitionShape::NewType { .. }
            | DefinitionShape::Enum { .. } => Ok(None),
        }
    }

    fn merge_fields_all_of(&self, shapes: &[AllOfItemShape]) -> Result<Vec<FieldShape>> {
        let mut field_shapes = vec![];
        for shape in shapes {
            field_shapes.append(&mut self.shape_item_to_fields(shape)?)
        }
        Ok(field_shapes)
    }

    fn shape_item_to_fields(&self, item_shape: &AllOfItemShape) -> Result<Vec<FieldShape>> {
        match item_shape {
            AllOfItemShape::Object(shapes) => Ok(shapes.clone()),
            AllOfItemShape::Ref(object) => {
                let shape = self.snapshot.find_type_definition(object)?;
                Ok(shape.field_shapes().to_vec())
            }
        }
    }
}
