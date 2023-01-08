mod transform_core;
use transform_core::transform_core;

mod transform_request_bodies;
use transform_request_bodies::transform_request_bodies;

mod transform_schemas;
use transform_schemas::transform_schemas;

use crate::conversions::v3_0::to_rust_type::components::core::CoreShape;
use crate::conversions::v3_0::to_rust_type::components::request_bodies::RequestBodiesShape;
use crate::conversions::v3_0::to_rust_type::components::schemas::{
    SchemasShape, TypeDefinitionShape, TypeShape,
};
use crate::conversions::v3_0::to_rust_type::contains_patch;
use crate::conversions::Error::ReferenceObjectNotFound;
use crate::conversions::Result;
use crate::targets::rust_type::{Definitions, Imports, ModDef, ModuleName, Modules, Package};
use openapi_types::v3_0::{ReferenceObject, SchemaObject};

#[derive(Clone, Debug)]
pub struct ComponentsShape {
    pub schemas: SchemasShape,
    pub request_bodies: RequestBodiesShape,
    pub core: CoreShape,
}

impl ComponentsShape {
    pub fn into_modules(self) -> Result<Modules> {
        let this = transform(self)?;
        let modules = vec![
            this.request_bodies.define()?,
            this.schemas.define()?,
            this.core.define()?,
        ]
        .into_iter()
        .flatten()
        .collect();

        Ok(modules)
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
        let defs = &self.schemas.root.defs;
        defs.iter()
            .filter_map(|shape| shape.as_type_definition())
            .find(|shape| shape.is_type_name(&name))
            .ok_or_else(|| ReferenceObjectNotFound(type_ref.to_string()))
    }

    pub fn any_type(&self, f: impl Fn(&TypeShape) -> bool) -> bool {
        self.schemas.root.defs.iter().any(|x| x.any_type(&f))
        // TODO: check self.request_bodies
    }
}

fn transform(shapes: ComponentsShape) -> Result<ComponentsShape> {
    let shapes = transform_schemas(shapes)?;
    let shapes = transform_request_bodies(shapes)?;
    let shapes = transform_core(shapes)?;
    Ok(shapes)
}

// TODO: delete this fn
pub fn create_module<A: Into<String>>(name: A, definitions: Definitions) -> Result<Option<ModDef>> {
    let mut imports = Imports::new();
    imports.set(vec![Package::Deserialize, Package::Serialize]);

    if definitions.iter().any(|x| x.any_type(contains_patch)) {
        imports.set(Package::Patch { depth: 1 });
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
