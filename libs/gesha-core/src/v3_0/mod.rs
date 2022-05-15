use crate::yaml_wrapper::{YamlMap, YamlValue};
use crate::Error::IncompatibleVersion;
use crate::OpenApiDocument;
use openapi_types::v3_0::{
    Document, HttpStatusCode, InfoObject, OperationObject, PathFieldName, PathItemObject,
    PathsObject, ResponseCase, ResponseObject, ResponsesObject,
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

fn to_openapi_version(s: String) -> crate::Result<String> {
    if !s.starts_with("3.0.") {
        return Err(IncompatibleVersion);
    }
    Ok(s)
}

fn to_info(mut map: YamlMap) -> crate::Result<InfoObject> {
    let info = InfoObject {
        title: map.remove("title")?,
    };
    Ok(info)
}

fn to_paths_object(map: YamlMap) -> crate::Result<PathsObject> {
    let tuples = map
        .into_iter()
        .map(reify_entry)
        .collect::<crate::Result<Vec<(String, YamlMap)>>>()?
        .into_iter()
        .map(to_path_pair)
        .collect::<crate::Result<Vec<(PathFieldName, PathItemObject)>>>()?;

    Ok(PathsObject::new(tuples))
}

fn reify_entry(kv: crate::Result<(YamlValue, YamlValue)>) -> crate::Result<(String, YamlMap)> {
    match kv {
        Ok((k, v)) => Ok((k.try_into()?, v.try_into()?)),
        Err(e) => Err(e),
    }
}

fn to_path_pair(kv: (String, YamlMap)) -> crate::Result<(PathFieldName, PathItemObject)> {
    let (field, map) = kv;
    Ok((PathFieldName::new(field), to_path_item_object(map)?))
}

fn to_path_item_object(mut map: YamlMap) -> crate::Result<PathItemObject> {
    let get = map
        .remove_if_exists("get")?
        .map(to_operation_object)
        .transpose()?;

    let post = map
        .remove_if_exists("post")?
        .map(to_operation_object)
        .transpose()?;

    let obj = PathItemObject { get, post };
    Ok(obj)
}

fn to_operation_object(mut map: YamlMap) -> crate::Result<OperationObject> {
    let responses = to_responses_object(map.remove("responses")?)?;
    Ok(OperationObject { responses })
}

fn to_responses_object(map: YamlMap) -> crate::Result<ResponsesObject> {
    let tuples = map
        .into_iter()
        .map(reify_entry)
        .collect::<crate::Result<Vec<(String, YamlMap)>>>()?
        .into_iter()
        .map(to_response_pair)
        .collect::<crate::Result<Vec<(HttpStatusCode, ResponseCase)>>>()?;

    let default = None;
    Ok(ResponsesObject::new(tuples, default))
}

fn to_response_pair(kv: (String, YamlMap)) -> crate::Result<(HttpStatusCode, ResponseCase)> {
    let (field, map) = kv;
    Ok((to_http_status_code(field)?, to_response_case(map)?))
}

fn to_http_status_code(_v: String) -> crate::Result<HttpStatusCode> {
    // TODO:
    Ok(HttpStatusCode::OK)
}

fn to_response_case(_map: YamlMap) -> crate::Result<ResponseCase> {
    // TODO:
    Ok(ResponseCase::Response(ResponseObject {}))
}
