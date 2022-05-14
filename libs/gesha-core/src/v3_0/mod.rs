use crate::Error::Incompatible;
use crate::{Error, OpenApiDocument};
use openapi_types::v3_0::{
    Document, InfoObject, OperationObject, PathFieldName, PathItemObject, PathsObject,
    ResponsesObject,
};
use yaml_rust::Yaml;

/// return Error::Incompatible if not supported version.
pub fn to_document(yaml: Yaml) -> crate::Result<OpenApiDocument> {
    let openapi = match &yaml["openapi"] {
        Yaml::String(version) if version.starts_with("3.0.") => version.to_owned(),
        Yaml::String(_) => return Err(Incompatible),
        _ => return Err(Error::cannot_parse("'openapi' must be string")),
    };
    println!("openapi...{:#?}", openapi);

    let document = Document {
        openapi: "3.0.0".to_string(),
        info: InfoObject {
            title: "sample title".to_string(),
        },
        paths: PathsObject::new(vec![(
            PathFieldName::new("/pets"),
            PathItemObject {
                get: Some(OperationObject {
                    responses: ResponsesObject::new(vec![], None),
                }),
                post: None,
            },
        )]),
    };
    Ok(OpenApiDocument::V3_0(document))
}
