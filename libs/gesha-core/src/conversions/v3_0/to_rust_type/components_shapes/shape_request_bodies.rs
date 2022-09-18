use crate::conversions::v3_0::to_rust_type::components_shapes::{create_module, ComponentsShapes};
use crate::conversions::v3_0::to_rust_type::from_request_bodies::{
    DefinitionShape, MediaTypeShape,
};
use crate::conversions::Result;
use crate::targets::rust_type::{
    Definition, EnumVariant, EnumVariantName, Module, RequestBodyDef, TypeHeader,
};

impl ComponentsShapes {
    pub fn shape_request_bodies(&mut self) -> Result<Option<Module>> {
        println!("{:#?}", self.request_bodies);

        let definitions = self
            .request_bodies
            .clone()
            .into_iter()
            .map(to_definition)
            .collect();

        create_module("request_bodies", definitions)
    }
}

fn to_definition(shape: DefinitionShape) -> Definition {
    let header = TypeHeader::new(shape.name.to_string(), shape.doc_comments);

    let variants = shape
        .contents
        .into_iter()
        .filter_map(|s| match s.media_type {
            MediaTypeShape::Unsupported(_) => None,
            // TODO
            x => Some(EnumVariant::tuple(EnumVariantName::new(x), vec![])),
        })
        .collect();

    RequestBodyDef::new(header, variants).into()
}
