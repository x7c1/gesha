use crate::conversions::{reify_entry, Result};
use crate::yaml::YamlMap;
use openapi_types::v3_0::{
    HttpStatusCode, OperationObject, PathFieldName, PathItemObject, PathsObject, ResponseCase,
    ResponseObject, ResponsesObject,
};

pub(super) fn to_paths_object(map: YamlMap) -> Result<PathsObject> {
    let tuples = map
        .into_iter()
        .map(reify_entry)
        .collect::<Result<Vec<(String, YamlMap)>>>()?
        .into_iter()
        .map(to_path_pair)
        .collect::<Result<Vec<(PathFieldName, PathItemObject)>>>()?;

    Ok(PathsObject::new(tuples))
}

fn to_path_pair(kv: (String, YamlMap)) -> Result<(PathFieldName, PathItemObject)> {
    let (field, map) = kv;
    Ok((PathFieldName::new(field), to_path_item_object(map)?))
}

fn to_path_item_object(mut map: YamlMap) -> Result<PathItemObject> {
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

fn to_operation_object(mut map: YamlMap) -> Result<OperationObject> {
    let responses = to_responses_object(map.remove("responses")?)?;
    Ok(OperationObject { responses })
}

fn to_responses_object(map: YamlMap) -> Result<ResponsesObject> {
    let tuples = map
        .into_iter()
        .map(reify_entry)
        .collect::<Result<Vec<(String, YamlMap)>>>()?
        .into_iter()
        .map(to_response_pair)
        .collect::<Result<Vec<(HttpStatusCode, ResponseCase)>>>()?;

    let default = None;
    Ok(ResponsesObject::new(tuples, default))
}

fn to_response_pair(kv: (String, YamlMap)) -> Result<(HttpStatusCode, ResponseCase)> {
    let (field, map) = kv;
    Ok((to_http_status_code(field)?, to_response_case(map)?))
}

fn to_http_status_code(_v: String) -> Result<HttpStatusCode> {
    // TODO:
    Ok(HttpStatusCode::OK)
}

fn to_response_case(_map: YamlMap) -> Result<ResponseCase> {
    // TODO:
    Ok(ResponseCase::Response(ResponseObject {}))
}
