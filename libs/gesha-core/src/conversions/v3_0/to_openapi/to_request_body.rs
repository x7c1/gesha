use crate::conversions::reify::reify_entry;
use crate::conversions::Result;
use crate::yaml::YamlMap;
use indexmap::IndexMap;
use openapi_types::v3_0::{
    ComponentName, MediaTypeKey, MediaTypeObject, RequestBodyCase, RequestBodyContent,
    RequestBodyObject,
};

pub(super) fn to_request_body_pair(
    kv: (String, YamlMap),
) -> Result<(ComponentName, RequestBodyCase)> {
    let (name, map) = kv;
    Ok((ComponentName::new(name), to_request_body_case(map)?))
}

fn to_request_body_case(mut map: YamlMap) -> Result<RequestBodyCase> {
    let case = match map.remove_if_exists::<String>("$ref")? {
        Some(_reference) => unimplemented!(),
        None => RequestBodyCase::RequestBody(Box::new(to_request_body_object(map)?)),
    };
    Ok(case)
}

fn to_request_body_object(mut map: YamlMap) -> Result<RequestBodyObject> {
    let content = map.remove("content").map(to_request_body_content)??;
    Ok(RequestBodyObject {
        description: map.remove_if_exists("description")?,
        content,
        required: map.remove_if_exists("required")?.unwrap_or(false),
    })
}

fn to_request_body_content(map: YamlMap) -> Result<RequestBodyContent> {
    let pairs = map
        .into_iter()
        .map(reify_entry)
        .collect::<Result<Vec<(String, YamlMap)>>>()?
        .into_iter()
        .map(to_request_body_content_pair)
        .collect::<Result<IndexMap<MediaTypeKey, MediaTypeObject>>>()?;

    Ok(RequestBodyContent::new(pairs))
}

fn to_request_body_content_pair(kv: (String, YamlMap)) -> Result<(MediaTypeKey, MediaTypeObject)> {
    let (name, map) = kv;
    Ok((MediaTypeKey::new(name), to_media_type_object(map)?))
}

fn to_media_type_object(mut _map: YamlMap) -> Result<MediaTypeObject> {
    Ok(MediaTypeObject {})
}
