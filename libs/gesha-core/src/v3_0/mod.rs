use crate::yaml_wrapper::YamlMap;
use crate::Error::IncompatibleVersion;
use crate::OpenApiDocument;
use openapi_types::v3_0::{
    Document, InfoObject, OperationObject, PathFieldName, PathItemObject, PathsObject,
    ResponsesObject,
};

/// return Error::IncompatibleVersion if not supported version.
pub fn to_document(mut map: YamlMap) -> crate::Result<OpenApiDocument> {
    let document = Document {
        openapi: to_openapi_version(map.remove("openapi")?)?,
        info: to_info(map.remove("info")?)?,
        paths: to_paths_object(map.remove("paths")?)?,
    };
    Ok(OpenApiDocument::V3_0(document))
}

pub fn to_openapi_version(s: String) -> crate::Result<String> {
    if !s.starts_with("3.0.") {
        return Err(IncompatibleVersion);
    }
    Ok(s)
}

pub fn to_info(mut map: YamlMap) -> crate::Result<InfoObject> {
    let info = InfoObject {
        title: map.remove("title")?,
    };
    Ok(info)
}

pub fn to_paths_object(_map: YamlMap) -> crate::Result<PathsObject> {
    // TODO: convert _map to PathsObject
    Ok(PathsObject::new(vec![(
        PathFieldName::new("/pets"),
        PathItemObject {
            get: Some(OperationObject {
                responses: ResponsesObject::new(vec![], None),
            }),
            post: None,
        },
    )]))
}
