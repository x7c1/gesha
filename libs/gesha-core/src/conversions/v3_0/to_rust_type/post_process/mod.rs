mod resolve_all_of;
mod resolve_ref;

use super::ComponentsShapes;
use crate::conversions::Result;

pub(super) struct PostProcessor {
    original: ComponentsShapes,
}

impl PostProcessor {
    pub fn run(modules: &mut ComponentsShapes) -> Result<()> {
        let this = Self {
            original: modules.clone(),
        };
        // 1st process : resolve allOf
        modules
            .schemas
            .iter_mut()
            .try_for_each(|x| this.resolve_all_of(x))?;

        // 2nd process : resolve $ref
        modules
            .schemas
            .iter_mut()
            .try_for_each(|x| this.resolve_ref(x))?;

        Ok(())
    }
}
