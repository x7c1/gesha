mod expand_inline_schemas;
mod resolve_all_of;
mod resolve_ref;

use crate::conversions::v3_0::to_rust_type::components_shapes::ComponentsShapes;
use crate::conversions::v3_0::to_rust_type::from_schemas::DefinitionShape;
use crate::conversions::Result;
use crate::targets::rust_type::Definitions;

pub struct PostProcessor {
    original: ComponentsShapes,
}

impl PostProcessor {
    pub fn new(original: ComponentsShapes) -> Self {
        Self { original }
    }

    pub fn run(
        &self,
        shapes: &mut Vec<DefinitionShape>,
        prefix: &'static str,
    ) -> Result<Definitions> {
        // 1st process : expand inline schemas
        self.process_inline_schemas(shapes)?;

        // 2nd process : resolve allOf
        self.process_all_of(shapes)?;

        // 3rd process : resolve $ref
        self.process_ref(prefix, shapes)
    }
}
