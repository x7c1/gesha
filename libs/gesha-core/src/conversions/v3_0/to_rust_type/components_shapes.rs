use crate::conversions::v3_0::to_rust_type::DefinitionShape;
use crate::conversions::v3_0::to_rust_type::DefinitionShape::{Fixed, InProcess};
use crate::conversions::Error::PostProcessBroken;
use crate::conversions::Result;
use crate::targets::rust_type::{Definition, Module, ModuleName, Modules};
use openapi_types::v3_0::ReferenceObject;

#[derive(Clone, Debug)]
pub struct ComponentsShapes {
    pub(super) schemas: Vec<DefinitionShape>,
}

impl ComponentsShapes {
    pub fn into_modules(self) -> Result<Modules> {
        let schemas_definitions = self
            .schemas
            .into_iter()
            .map(to_definition)
            .collect::<Result<Vec<Definition>>>()?;

        let schemas = Module::new(ModuleName::new("schemas"), schemas_definitions);
        let mut modules = vec![schemas];

        let is_patch_used = modules.iter().any(|x| x.is_patch_used);
        if is_patch_used {
            modules.push(Module::init(
                ModuleName::new("core"),
                vec![Definition::generate_patch()],
            ));
        }
        Ok(Modules::new(modules))
    }
}

fn to_definition(shape: DefinitionShape) -> Result<Definition> {
    match shape {
        Fixed(def) => Ok(def),
        InProcess(process) => Err(PostProcessBroken {
            detail: format!("post-process has been left.\n{:#?}", process),
        }),
    }
}

impl ComponentsShapes {
    pub(super) fn find_definition(&self, object: &ReferenceObject) -> Result<&DefinitionShape> {
        // TODO: support other locations like 'components/responses' etc
        find_shape("#/components/schemas/", &self.schemas, object).ok_or_else(|| unimplemented!())
    }
}

fn find_shape<'a, 'b>(
    prefix: &str,
    defs: &'a [DefinitionShape],
    target: &'b ReferenceObject,
) -> Option<&'a DefinitionShape> {
    let type_ref = target.as_ref();
    if type_ref.starts_with(prefix) {
        let name = type_ref.replace(prefix, "");
        defs.iter().find(|shape| shape.is_struct_name(&name))
    } else {
        None
    }
}
