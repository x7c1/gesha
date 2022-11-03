mod shape_request_bodies;
mod shape_schemas;

use crate::conversions::v3_0::to_rust_type::from_request_bodies::MediaTypeShape;
use crate::conversions::v3_0::to_rust_type::{contains_patch, from_request_bodies, from_schemas};
use crate::conversions::Result;
use crate::targets::rust_type::{
    Definition, MediaTypeDef, Module, ModuleName, Modules, PresetDef, UseStatement,
};

#[derive(Clone, Debug)]
pub struct ComponentsShapes {
    pub(super) schemas: Vec<from_schemas::DefinitionShape>,
    pub(super) request_bodies: Vec<from_request_bodies::DefinitionShape>,
}

impl ComponentsShapes {
    pub fn into_modules(mut self) -> Result<Modules> {
        let modules = vec![self.shape_request_bodies()?, self.shape_schemas_module()?]
            .into_iter()
            .flatten()
            .collect();

        Ok(self.create_modules(modules))
    }

    fn create_modules(&self, modules: Vec<Module>) -> Modules {
        let mut modules = Modules::new(modules);
        if let Some(core) = self.create_core_module(&modules) {
            modules.push(core);
        }
        modules
    }

    fn create_core_module(&self, modules: &Modules) -> Option<Module> {
        let mut core_defs = vec![];
        let mut imports = default_imports();

        if modules.any_type(contains_patch) {
            core_defs.push(PresetDef::patch().into());
            imports.push(UseStatement::new("serde::Deserializer"));
            imports.push(UseStatement::new("serde::Serializer"));
            imports.push(UseStatement::new("serde::ser::Error"));
        }

        if let Some(media_type) = self.create_media_type_def() {
            core_defs.push(PresetDef::MediaType(media_type).into())
        }

        if core_defs.is_empty() {
            None
        } else {
            let module = Module::new(ModuleName::new("core"), core_defs, imports);
            Some(module)
        }
    }

    fn create_media_type_def(&self) -> Option<MediaTypeDef> {
        println!("request : {:#?}", self.request_bodies);

        // TODO:
        self.request_bodies.iter().for_each(|def| {
            def.contents
                .iter()
                .for_each(|content| match content.media_type {
                    MediaTypeShape::ApplicationJson => {}
                    MediaTypeShape::Unsupported(_) => {}
                })
        });

        Some(MediaTypeDef)
    }
}

fn create_module<A: Into<String>>(name: A, definitions: Vec<Definition>) -> Result<Option<Module>> {
    let mut imports = default_imports();
    if definitions.iter().any(|x| x.any_type(contains_patch)) {
        imports.push(UseStatement::new("super::core::Patch"));
    }
    if definitions.is_empty() {
        Ok(None)
    } else {
        let module = Module::new(ModuleName::new(name), definitions, imports);
        Ok(Some(module))
    }
}

fn default_imports() -> Vec<UseStatement> {
    vec![
        UseStatement::new("serde::Deserialize"),
        UseStatement::new("serde::Serialize"),
    ]
}
