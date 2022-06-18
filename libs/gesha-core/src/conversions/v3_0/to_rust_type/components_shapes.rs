use crate::conversions::v3_0::to_rust_type::DefinitionShape;
use crate::conversions::v3_0::to_rust_type::DefinitionShape::{Fixed, InProcess};
use crate::conversions::Error::RequirePostProcess;
use crate::conversions::Result;
use crate::targets::rust_type::{Definition, ModuleName, Modules};
use indexmap::indexmap;
use openapi_types::v3_0::ReferenceObject;

#[derive(Clone, Debug)]
pub struct ComponentsShapes {
    pub(super) schemas: Vec<DefinitionShape>,
}

impl ComponentsShapes {
    pub fn into_modules(self) -> Result<Modules> {
        let schemas = self
            .schemas
            .into_iter()
            .map(to_definition)
            .collect::<Result<Vec<Definition>>>()?;

        Ok(indexmap! {
             ModuleName::new("schemas") => schemas,
        })
    }
}

fn to_definition(shape: DefinitionShape) -> Result<Definition> {
    match shape {
        Fixed(def) => Ok(def),
        InProcess(process) => Err(RequirePostProcess {
            detail: format!("{:#?}", process),
        }),
    }
}

impl ComponentsShapes {
    pub(super) fn find_definition(&self, object: &ReferenceObject) -> Result<&DefinitionShape> {
        // TODO: support locations other than 'schemas'
        let prefix = "#/components/schemas/";
        let type_ref = object.as_ref();
        let def = if type_ref.starts_with(prefix) {
            let name = type_ref.replace(prefix, "");
            let defs = &self.schemas;
            defs.iter().find(|shape| shape.is_struct_name(&name))
        } else {
            unimplemented!()
        };
        def.ok_or_else(|| unimplemented!())
    }
}
