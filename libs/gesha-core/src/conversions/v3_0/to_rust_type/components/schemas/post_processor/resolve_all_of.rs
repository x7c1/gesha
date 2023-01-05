use crate::conversions::v3_0::to_rust_type::components::schemas::{
    AllOfItemShape, AllOfShape, DefinitionShape, FieldShape, ModShape, PostProcessor, StructShape,
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
            DefinitionShape::Mod(ModShape { name, defs }) => Ok(DefinitionShape::Mod(ModShape {
                name,
                defs: self.process_all_of(defs)?,
            })),
            DefinitionShape::Struct { .. }
            | DefinitionShape::NewType { .. }
            | DefinitionShape::Enum { .. } => Ok(def_shape),
        }
    }

    fn merge_fields_all_of(&self, shapes: Vec<AllOfItemShape>) -> Result<Vec<FieldShape>> {
        let fields = shapes
            .into_iter()
            .map(|x| self.shape_item_to_fields(x))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();

        Ok(fields)
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
