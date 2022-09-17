use crate::conversions::v3_0::to_rust_type::components_shapes::{create_module, ComponentsShapes};
use crate::conversions::v3_0::to_rust_type::from_schemas;
use crate::conversions::v3_0::to_rust_type::from_schemas::PostProcessor;
use crate::conversions::Result;
use crate::targets::rust_type::Module;
use openapi_types::v3_0::{ReferenceObject, SchemaObject};

impl ComponentsShapes {
    pub fn create_schemas_module(&mut self) -> Result<Module> {
        let processor = PostProcessor::new(self.clone());
        create_module(
            "schemas",
            processor.run(&mut self.schemas, "#/components/schemas/")?,
        )
    }

    pub fn find_definition(
        &self,
        object: &ReferenceObject<SchemaObject>,
    ) -> Result<&from_schemas::DefinitionShape> {
        // TODO: support other locations like 'components/responses' etc
        find_shape("#/components/schemas/", &self.schemas, object).ok_or_else(|| unimplemented!())
    }
}

fn find_shape<'a, 'b>(
    prefix: &str,
    defs: &'a [from_schemas::DefinitionShape],
    target: &'b ReferenceObject<SchemaObject>,
) -> Option<&'a from_schemas::DefinitionShape> {
    let type_ref = target.as_ref();
    if type_ref.starts_with(prefix) {
        let name = type_ref.replace(prefix, "");
        defs.iter().find(|shape| shape.is_type_name(&name))
    } else {
        None
    }
}
