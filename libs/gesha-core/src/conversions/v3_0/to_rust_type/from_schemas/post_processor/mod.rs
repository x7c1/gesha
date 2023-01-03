mod expand_inline_schemas;
mod resolve_all_of;
mod resolve_ref;
mod to_definitions;

use crate::conversions::v3_0::to_rust_type::components_shapes::ComponentsShapes;
use crate::conversions::v3_0::to_rust_type::from_schemas::DefinitionShape;
use crate::conversions::Result;
use crate::targets::rust_type::Definitions;

pub struct PostProcessor {
    snapshot: ComponentsShapes,
}

impl PostProcessor {
    pub fn new(snapshot: ComponentsShapes) -> Self {
        Self { snapshot }
    }

    pub fn run(
        &mut self,
        shapes: &mut Vec<DefinitionShape>,
        prefix: &'static str,
    ) -> Result<Definitions> {
        // 1st process : expand inline schemas
        self.process_inline_schemas(shapes)?;
        self.snapshot.schemas = shapes.clone();

        // 2nd process : resolve allOf
        self.process_all_of(shapes)?;
        self.snapshot.schemas = shapes.clone();

        // 3rd process : resolve $ref
        *shapes = self.process_ref(prefix, shapes.clone())?;

        self.to_definitions(shapes)
    }
}
