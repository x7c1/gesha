mod resolve_all_of;
mod resolve_ref;

use super::ComponentsShapes;
use crate::conversions::Result;
use crate::targets::rust_type::Definition;

pub(super) struct PostProcessor {
    original: ComponentsShapes,
}

impl PostProcessor {
    pub fn run2(modules: &mut ComponentsShapes) -> Result<Vec<Definition>> {
        let this = Self {
            original: modules.clone(),
        };
        // 1st process : resolve allOf
        this.process_all_of(modules)?;

        // 2nd process : resolve $ref
        this.process_ref2(modules)
    }

    // pub fn run(modules: &mut ComponentsShapes) -> Result<()> {
    //     let this = Self {
    //         original: modules.clone(),
    //     };
    //     // 1st process : resolve allOf
    //     this.process_all_of(modules)?;
    //
    //     // 2nd process : resolve $ref
    //     this.process_ref(modules)
    // }
}
