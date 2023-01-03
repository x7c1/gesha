use crate::conversions::v3_0::to_rust_type::from_schemas::post_processor::PostProcessor;
use crate::conversions::v3_0::to_rust_type::from_schemas::{
    AllOfItemShape, AllOfShape, DefinitionShape, FieldShape, StructShape,
};
use crate::conversions::Result;

impl PostProcessor {
    pub fn process_all_of(&self, shapes: Vec<DefinitionShape>) -> Result<Vec<DefinitionShape>> {
        shapes.into_iter().map(|x| self.shape_all_of(x)).collect()
    }

    fn shape_all_of(&self, def_shape: DefinitionShape) -> Result<DefinitionShape> {
        match def_shape {
            DefinitionShape::AllOf(AllOfShape { header, items }) => {
                Ok(DefinitionShape::Struct(StructShape {
                    header,
                    fields: self.merge_fields_all_of(items)?,
                }))
            }
            DefinitionShape::Mod { name, defs, .. } => Ok(DefinitionShape::Mod {
                name,
                defs: self.process_all_of(defs)?,
            }),
            DefinitionShape::Struct { .. }
            | DefinitionShape::NewType { .. }
            | DefinitionShape::Enum { .. } => Ok(def_shape),
        }
    }

    fn merge_fields_all_of(&self, shapes: Vec<AllOfItemShape>) -> Result<Vec<FieldShape>> {
        let mut field_shapes = vec![];
        for shape in shapes {
            field_shapes.append(&mut self.shape_item_to_fields(shape)?)
        }
        Ok(field_shapes)
    }

    fn shape_item_to_fields(&self, item_shape: AllOfItemShape) -> Result<Vec<FieldShape>> {
        match item_shape {
            AllOfItemShape::Object(shapes) => Ok(shapes),
            AllOfItemShape::Ref(object) => {
                let shape = self.snapshot.find_type_definition(&object)?;
                Ok(shape.field_shapes().to_vec())
            }
        }
    }
}
