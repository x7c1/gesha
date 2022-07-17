use crate::conversions::v3_0::to_rust_type::components_shapes::ComponentsShapes;
use crate::conversions::v3_0::to_rust_type::post_process::PostProcessor;
// use crate::conversions::v3_0::to_rust_type::DefinitionShape::InProcess;
use crate::conversions::v3_0::to_rust_type::{AllOfItemShape, DefinitionShape, FieldShape};
use crate::conversions::Result;

impl PostProcessor {
    pub(super) fn process_all_of(&self, modules: &mut ComponentsShapes) -> Result<()> {
        modules
            .schemas
            .iter_mut()
            .try_for_each(|x| self.resolve_all_of(x))
    }

    // fn resolve_all_of(&self, shape: &mut DefinitionShape) -> Result<()> {
    //     if let InProcess(process) = shape {
    //         if let Some(processed) = self.shape_all_of(process)? {
    //             *shape = processed;
    //         }
    //     }
    //     Ok(())
    // }

    // fn shape_all_of(&self, process: &mut PostProcess) -> Result<Option<DefinitionShape>> {
    //     match process {
    //         PostProcess::AllOf { header, shapes } => {
    //             let process = PostProcess::Struct {
    //                 header: header.clone(),
    //                 shapes: self.merge_fields_all_of(shapes)?,
    //             };
    //             Ok(Some(process.into()))
    //         }
    //         PostProcess::Struct { .. } => {
    //             // shaped in next process.
    //             Ok(None)
    //         }
    //         PostProcess::NewType { .. } => {
    //             // shaped in next process.
    //             Ok(None)
    //         }
    //     }
    // }

    fn resolve_all_of(&self, shape: &mut DefinitionShape) -> Result<()> {
        if let Some(processed) = self.shape_all_of(shape)? {
            *shape = processed;
        }
        Ok(())
    }

    fn shape_all_of(&self, process: &mut DefinitionShape) -> Result<Option<DefinitionShape>> {
        match process {
            DefinitionShape::AllOf { header, shapes } => {
                let process = DefinitionShape::Struct {
                    header: header.clone(),
                    shapes: self.merge_fields_all_of(shapes)?,
                };
                Ok(Some(process.into()))
            }
            // shaped in next processes.
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
                let fields = self.original.find_definition(object)?.field_shapes();
                Ok(fields)
            }
        }
    }
}
