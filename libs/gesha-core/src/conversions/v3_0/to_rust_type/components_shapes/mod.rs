mod shape_schemas;

use crate::conversions::v3_0::to_rust_type::{contains_patch, from_request_bodies, from_schemas};
use crate::conversions::Result;
use crate::targets::rust_type::{Definition, Module, ModuleName, Modules, PresetDef, UseStatement};

#[derive(Clone, Debug)]
pub struct ComponentsShapes {
    pub(super) schemas: Vec<from_schemas::DefinitionShape>,
    pub(super) request_bodies: Vec<from_request_bodies::DefinitionShape>,
}

impl ComponentsShapes {
    pub fn into_modules(mut self) -> Result<Modules> {
        let schemas = self.create_schemas_module()?;
        let modules = create_modules(vec![schemas]);
        Ok(modules)
    }
}

fn create_module<A: Into<String>>(name: A, definitions: Vec<Definition>) -> Result<Module> {
    let mut imports = default_imports();
    if definitions.iter().any(|x| x.any_type(contains_patch)) {
        imports.push(UseStatement::new("super::core::Patch"));
    }
    let module = Module::new(ModuleName::new(name), definitions, imports);
    Ok(module)
}

fn create_modules(modules: Vec<Module>) -> Modules {
    let mut modules = Modules::new(modules);
    if let Some(core) = create_core_module(&modules) {
        modules.push(core);
    }
    modules
}

fn create_core_module(modules: &Modules) -> Option<Module> {
    let mut core_defs = vec![];
    let mut imports = default_imports();

    if modules.any_type(contains_patch) {
        core_defs.push(PresetDef::patch().into());
        imports.push(UseStatement::new("serde::Deserializer"));
        imports.push(UseStatement::new("serde::Serializer"));
        imports.push(UseStatement::new("serde::ser::Error"));
    }

    if core_defs.is_empty() {
        None
    } else {
        let module = Module::new(ModuleName::new("core"), core_defs, imports);
        Some(module)
    }
}

fn default_imports() -> Vec<UseStatement> {
    vec![
        UseStatement::new("serde::Deserialize"),
        UseStatement::new("serde::Serialize"),
    ]
}
