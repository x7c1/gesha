use crate::conversions::v3_0::to_rust_type::components_shapes::ComponentsShapes;
use crate::conversions::v3_0::to_rust_type::post_process::PostProcessor;
use crate::conversions::v3_0::to_rust_type::DefinitionShape::InProcess;
use crate::conversions::v3_0::to_rust_type::{
    AllOfItemShape, DefinitionShape, FieldShape, PostProcess,
};
use crate::conversions::Result;
use crate::targets::rust_type::TypeHeader;

impl PostProcessor {
    pub(super) fn process_all_of(&self, modules: &mut ComponentsShapes) -> Result<()> {
        modules
            .schemas
            .iter_mut()
            .try_for_each(|x| self.resolve_all_of(x))
    }

    fn resolve_all_of(&self, shape: &mut DefinitionShape) -> Result<()> {
        if let InProcess(process) = shape {
            if let Some(processed) = self.shape_all_of(process)? {
                *shape = processed;
            }
        }
        Ok(())
    }

    fn shape_all_of(&self, process: &mut PostProcess) -> Result<Option<DefinitionShape>> {
        match process {
            PostProcess::AllOf {
                struct_name,
                shapes,
            } => {
                let doc_comments = Some("TODO: extract doc_comments".to_string());
                let header = TypeHeader::new(struct_name.clone(), doc_comments);
                let process = PostProcess::Struct {
                    header,
                    shapes: self.merge_fields_all_of(shapes)?,
                };
                Ok(Some(process.into()))
            }
            PostProcess::Struct { .. } => {
                // shaped in next process.
                Ok(None)
            }
            PostProcess::NewType { .. } => {
                // shaped in next process.
                Ok(None)
            }
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
