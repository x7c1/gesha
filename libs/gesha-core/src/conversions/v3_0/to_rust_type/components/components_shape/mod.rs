mod define_core;
use define_core::define_core;

mod define_request_bodies;
use define_request_bodies::define_request_bodies;

mod define_schemas;
use define_schemas::define_schemas;

mod transform_core;
use transform_core::transform_core;

mod transform_request_bodies;
use transform_request_bodies::transform_request_bodies;

mod transform_schemas;
use transform_schemas::transform_schemas;

use crate::conversions::v3_0::to_rust_type::components::core::CoreShape;
use crate::conversions::v3_0::to_rust_type::components::request_bodies;
use crate::conversions::v3_0::to_rust_type::components::request_bodies::RequestBodiesShape;
use crate::conversions::v3_0::to_rust_type::components::schemas::{
    AllOfItemShape, AllOfShape, DefinitionShape, FieldShape, SchemasShape, StructShape,
    TypeDefinitionShape, TypeShape,
};
use crate::conversions::v3_0::to_rust_type::contains_patch;
use crate::conversions::Error::ReferenceObjectNotFound;
use crate::conversions::Result;
use crate::targets::rust_type::{
    Definitions, EnumVariantName, ErrorDef, ErrorVariant, Imports, MediaTypeDef, ModDef,
    ModuleName, Modules, Package, PresetDef,
};
use indexmap::IndexMap;
use openapi_types::v3_0::{ReferenceObject, SchemaObject};

#[derive(Clone, Debug, Default)]
pub struct ComponentsShape {
    pub schemas: SchemasShape,
    pub request_bodies: RequestBodiesShape,
    pub core: CoreShape,
}

impl ComponentsShape {
    pub fn into_modules(self) -> Result<Modules> {
        let this = transform(self)?;
        let modules = vec![
            define_request_bodies(this.request_bodies)?,
            define_schemas(this.schemas)?,
            define_core(this.core)?,
        ]
        .into_iter()
        .flatten()
        .collect();

        add_core_mod(modules)
    }

    pub fn find_type_definition(
        &self,
        object: &ReferenceObject<SchemaObject>,
    ) -> Result<TypeDefinitionShape> {
        let prefix = "#/components/schemas/";
        let type_ref = object.as_ref();
        if !type_ref.starts_with(prefix) {
            unimplemented!()
        }
        let name = type_ref.replace(prefix, "");
        self.schemas
            .iter()
            .filter_map(|shape| shape.as_type_definition())
            .find(|shape| shape.is_type_name(&name))
            .ok_or_else(|| ReferenceObjectNotFound(type_ref.to_string()))
    }

    pub fn any_type(&self, f: impl Fn(&TypeShape) -> bool) -> bool {
        self.schemas.iter().any(|x| x.any_type(&f))
        // TODO: check self.request_bodies
    }

    // fn create_core_module(&self, modules: &Modules) -> Option<ModDef> {
    //     let mut core_defs = Definitions::new();
    //     let mut imports = Imports::new();
    //     let mut error_def = ErrorDef::new();
    //
    //     if modules.any_type(contains_patch) {
    //         core_defs.set(PresetDef::Patch);
    //         imports.set(vec![
    //             Package::Deserialize,
    //             Package::Deserializer,
    //             Package::Serialize,
    //             Package::Serializer,
    //         ]);
    //     }
    //
    //     // if let Some(media_type) = self.create_media_type_def() {
    //     //     imports.set(vec![
    //     //         Package::Deserialize,
    //     //         Package::Display,
    //     //         Package::Formatter,
    //     //     ]);
    //     //     core_defs.set(PresetDef::MediaType(media_type));
    //     //     core_defs.set(PresetDef::FromJson);
    //     //     error_def.set(ErrorVariant::InvalidJson);
    //     //     error_def.set(ErrorVariant::UnsupportedMediaType);
    //     // }
    //
    //     if !error_def.is_empty() {
    //         core_defs.set(PresetDef::Error(error_def));
    //     }
    //
    //     if core_defs.is_empty() {
    //         None
    //     } else {
    //         let module = ModDef {
    //             name: ModuleName::new("core"),
    //             imports,
    //             defs: core_defs,
    //         };
    //         Some(module)
    //     }
    // }

    // fn create_media_type_def(&self) -> Option<MediaTypeDef> {
    //     let translator = self
    //         .request_bodies
    //         .iter()
    //         .flat_map(|def| def.translate_media_types())
    //         .collect::<IndexMap<EnumVariantName, &str>>();
    //
    //     if translator.is_empty() {
    //         None
    //     } else {
    //         Some(MediaTypeDef { translator })
    //     }
    // }
}

fn transform(shapes: ComponentsShape) -> Result<ComponentsShape> {
    let shapes = transform_schemas(shapes)?;
    let shapes = transform_request_bodies(shapes)?;
    let shapes = transform_core(shapes)?;
    Ok(shapes)
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

fn add_core_mod(mut modules: Modules) -> Result<Modules> {
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

    // if let Some(media_type) = self.create_media_type_def() {
    //     imports.set(vec![
    //         Package::Deserialize,
    //         Package::Display,
    //         Package::Formatter,
    //     ]);
    //     core_defs.set(PresetDef::MediaType(media_type));
    //     core_defs.set(PresetDef::FromJson);
    //     error_def.set(ErrorVariant::InvalidJson);
    //     error_def.set(ErrorVariant::UnsupportedMediaType);
    // }

    if !error_def.is_empty() {
        core_defs.set(PresetDef::Error(error_def));
    }

    if !core_defs.is_empty() {
        let module = ModDef {
            name: ModuleName::new("core"),
            imports,
            defs: core_defs,
        };
        modules.push(module);
    }
    Ok(modules)
}
