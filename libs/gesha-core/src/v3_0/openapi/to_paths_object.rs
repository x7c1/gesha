use crate::v3_0::openapi::reify_entry;
use crate::yaml_wrapper::YamlMap;
use openapi_types::v3_0::{
    HttpStatusCode, OperationObject, PathFieldName, PathItemObject, PathsObject, ResponseCase,
    ResponseObject, ResponsesObject,
};

pub fn to_paths_object(map: YamlMap) -> crate::Result<PathsObject> {
    let tuples = map
        .into_iter()
        .map(reify_entry)
        .collect::<crate::Result<Vec<(String, YamlMap)>>>()?
        .into_iter()
        .map(to_path_pair)
        .collect::<crate::Result<Vec<(PathFieldName, PathItemObject)>>>()?;

    Ok(PathsObject::new(tuples))
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
