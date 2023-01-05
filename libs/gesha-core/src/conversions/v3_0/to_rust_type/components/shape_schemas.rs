use super::schemas::PostProcessor;
use crate::conversions::v3_0::to_rust_type::components::{
    create_module, schemas, ComponentsShapes,
};
use crate::conversions::Error::ReferenceObjectNotFound;
use crate::conversions::Result;
use crate::targets::rust_type::ModDef;
use openapi_types::v3_0::{ReferenceObject, SchemaObject};

impl ComponentsShapes {
    pub fn shape_schemas_module(&self) -> Result<Option<ModDef>> {
        let mut processor = PostProcessor::new(self.clone());
        create_module(
            "schemas",
            processor.run(self.schemas.clone(), "#/components/schemas/")?,
        )
    }

    pub fn find_type_definition(
        &self,
        object: &ReferenceObject<SchemaObject>,
    ) -> Result<schemas::TypeDefinitionShape> {
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
}
