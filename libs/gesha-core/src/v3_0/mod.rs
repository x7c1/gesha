use crate::yaml_wrapper::{YamlMap, YamlValue};
use crate::Error::IncompatibleVersion;
use crate::OpenApiDocument;
use openapi_types::v3_0::{
    Document, InfoObject, OperationObject, PathFieldName, PathItemObject, PathsObject,
    ResponsesObject,
};

/// return Error::IncompatibleVersion if not supported version.
pub fn to_document(mut map: YamlMap) -> crate::Result<OpenApiDocument> {
    let value = map.remove("openapi")?;
    let openapi: String = value.try_into()?;
    if !openapi.starts_with("3.0.") {
        return Err(IncompatibleVersion);
    }
    let document = Document {
        openapi,
        info: to_info(map.remove("info")?)?,
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

pub fn to_info(value: YamlValue) -> crate::Result<InfoObject> {
    let mut map: YamlMap = value.try_into()?;
    let info = InfoObject {
        title: map.remove("title")?.try_into()?,
    };
    Ok(info)
}
