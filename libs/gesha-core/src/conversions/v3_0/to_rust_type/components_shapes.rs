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
        let modules = setup_modules(vec![create_module("schemas", self.schemas)?]);
        Ok(modules)
    }
}

fn create_module<A: Into<String>>(name: A, shapes: Vec<DefinitionShape>) -> Result<Module> {
    let definitions = shapes
        .into_iter()
        .map(to_definition)
        .collect::<Result<Vec<Definition>>>()?;

    let module = Module::new(ModuleName::new(name), definitions);
    Ok(module)
}

fn setup_modules(modules: Vec<Module>) -> Modules {
    let mut modules = Modules::setup(modules);
    let mut core_defs = vec![];

    if modules.is_patch_used() {
        core_defs.push(Definition::generate_patch());
    }
    if !core_defs.is_empty() {
        modules.push(Module::init(ModuleName::new("core"), core_defs))
    }
    modules
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
