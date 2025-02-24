use crate::v3_0::{
    HttpStatusCode, OperationObject, PathFieldName, PathItemObject, PathsObject, ResponseCase,
    ResponseObject, ResponsesObject,
};
use crate::yaml::{collect, YamlMap};
use crate::{Error, Result};

pub(super) fn to_paths_object(map: YamlMap) -> Result<PathsObject> {
    let (tuples, errors) = collect(to_path_pair)(map);

    // TODO: return error with PathsObject
    println!("detected errors: {:#?}", errors);

    Ok(PathsObject::new(tuples))
}

fn to_path_pair(kv: (String, YamlMap)) -> Result<(PathFieldName, PathItemObject)> {
    let (field, map) = kv;
    Ok((
        PathFieldName::new(&field),
        to_path_item_object(map).map_err(Error::with_key(field))?,
    ))
}

fn to_path_item_object(mut map: YamlMap) -> Result<PathItemObject> {
    let get = map
        .remove_if_exists("get")?
        .map(to_operation_object)
        .transpose()
        .map_err(Error::with_key("get"))?;

    let post = map
        .remove_if_exists("post")?
        .map(to_operation_object)
        .transpose()
        .map_err(Error::with_key("post"))?;

    let obj = PathItemObject { get, post };
    Ok(obj)
}

fn to_operation_object(mut map: YamlMap) -> Result<OperationObject> {
    let responses = map.remove("responses")?;
    let responses = to_responses_object(responses).map_err(Error::with_key("responses"))?;
    Ok(OperationObject { responses })
}

fn to_responses_object(map: YamlMap) -> Result<ResponsesObject> {
    let (tuples, errors) = collect(to_response_pair)(map);

    // TODO: return error with PathsObject
    println!("detected errors: {:#?}", errors);

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
