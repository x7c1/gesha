pub mod request_bodies;
pub mod schemas;

mod shape_request_bodies;
mod shape_schemas;

use crate::conversions::v3_0::to_rust_type::components::schemas::SchemasShape;
use crate::conversions::v3_0::to_rust_type::contains_patch;
use crate::conversions::Result;
use crate::targets::rust_type::{
    Definitions, EnumVariantName, ErrorDef, ErrorVariant, Imports, MediaTypeDef, ModDef,
    ModuleName, Modules, Package, PresetDef,
};
use indexmap::IndexMap;

#[derive(Clone, Debug)]
pub struct ComponentsShapes {
    pub(super) schemas: SchemasShape,
    pub(super) request_bodies: Vec<request_bodies::DefinitionShape>,
}

impl ComponentsShapes {
    pub fn into_modules(self) -> Result<Modules> {
        let modules = vec![self.shape_request_bodies()?, self.shape_schemas_module()?]
            .into_iter()
            .flatten()
            .collect();

        Ok(self.create_modules(modules))
    }

    fn create_modules(&self, modules: Vec<ModDef>) -> Modules {
        let mut modules = Modules::new(modules);
        if let Some(core) = self.create_core_module(&modules) {
            modules.push(core);
        }
        modules
    }

    fn create_core_module(&self, modules: &Modules) -> Option<ModDef> {
        let mut core_defs = Definitions::new();
        let mut imports = Imports::new();
        let mut error_def = ErrorDef::new();

        if modules.any_type(contains_patch) {
            core_defs.set(PresetDef::Patch);
            imports.set(vec![
                Package::Deserialize,
                Package::Deserializer,
                Package::Serialize,
                Package::Serializer,
            ]);
        }

        if let Some(media_type) = self.create_media_type_def() {
            imports.set(vec![
                Package::Deserialize,
                Package::Display,
                Package::Formatter,
            ]);
            core_defs.set(PresetDef::MediaType(media_type));
            core_defs.set(PresetDef::FromJson);
            error_def.set(ErrorVariant::InvalidJson);
            error_def.set(ErrorVariant::UnsupportedMediaType);
        }

        if !error_def.is_empty() {
            core_defs.set(PresetDef::Error(error_def));
        }

        if core_defs.is_empty() {
            None
        } else {
            let module = ModDef {
                name: ModuleName::new("core"),
                imports,
                defs: core_defs,
            };
            Some(module)
        }
    }

    fn create_media_type_def(&self) -> Option<MediaTypeDef> {
        let translator = self
            .request_bodies
            .iter()
            .flat_map(|def| def.translate_media_types())
            .collect::<IndexMap<EnumVariantName, &str>>();

        if translator.is_empty() {
            None
        } else {
            Some(MediaTypeDef { translator })
        }
    }
}

fn create_module<A: Into<String>>(name: A, definitions: Definitions) -> Result<Option<ModDef>> {
    let mut imports = Imports::new();
    imports.set(vec![Package::Deserialize, Package::Serialize]);

    if definitions.iter().any(|x| x.any_type(contains_patch)) {
        imports.set(Package::Patch);
    }
    if definitions.is_empty() {
        Ok(None)
    } else {
        let module = ModDef {
            name: ModuleName::new(name),
            imports,
            defs: definitions,
        };
        Ok(Some(module))
    }
}
