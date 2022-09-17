mod resolve_all_of;
mod resolve_ref;

use crate::conversions::v3_0::to_rust_type::components_shapes::ComponentsShapes;
use crate::conversions::v3_0::to_rust_type::from_schemas::DefinitionShape;
use crate::conversions::Result;
use crate::targets::rust_type::Definition;

pub struct PostProcessor {
    original: ComponentsShapes,
}

impl PostProcessor {
    pub fn new(original: ComponentsShapes) -> Self {
        Self { original }
    }

    pub fn run(
        &self,
        shapes: &mut [DefinitionShape],
        prefix: &'static str,
    ) -> Result<Vec<Definition>> {
        // 1st process : resolve allOf
        self.process_all_of(shapes)?;

        // 2nd process : resolve $ref
        self.process_ref(prefix, shapes)
    }
}
