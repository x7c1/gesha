use crate::error::{OutputMergeOps, OutputPairOps};
use crate::v3_0::{
    HttpStatusCode, OperationObject, PathFieldName, PathItemObject, PathsObject, ResponseCase,
    ResponseObject, ResponsesObject,
};
use crate::yaml::{collect, YamlMap};
use crate::{Error, OptionOutputOps, Output, Result};

pub(super) fn to_paths_object(map: YamlMap) -> Result<Output<PathsObject>> {
    let pairs = collect(to_path_pair)(map);
    let (tuples, errors) = pairs.merge().to_tuple();
    let object = PathsObject::new(tuples);
    Ok(Output::new(object, errors))
}

fn to_path_pair(kv: (String, YamlMap)) -> Result<Output<(PathFieldName, PathItemObject)>> {
    let (field, map) = kv;
    let pair = (
        PathFieldName::new(&field),
        to_path_item_object(map)?.map_errors(Error::with_key(field)),
    );
    Ok(pair.lift())
}

fn to_path_item_object(mut map: YamlMap) -> Result<Output<PathItemObject>> {
    let (get, errors1) = map
        .remove_if_exists("get")?
        .map(to_operation_object)
        .transpose()?
        .maybe()
        .map_errors(Error::with_key("get"))
        .to_tuple();

    let (post, errors2) = map
        .remove_if_exists("post")?
        .map(to_operation_object)
        .transpose()?
        .maybe()
        .map_errors(Error::with_key("post"))
        .to_tuple();

    let object = PathItemObject { get, post };
    Ok(Output::new(object, errors1).append(errors2))
}

fn to_operation_object(mut map: YamlMap) -> Result<Output<OperationObject>> {
    let responses = map.remove("responses")?;
    let (responses, errors) = to_responses_object(responses)?
        .map_errors(Error::with_key("responses"))
        .to_tuple();

    let object = OperationObject { responses };
    Ok(Output::new(object, errors))
}

fn to_responses_object(map: YamlMap) -> Result<Output<ResponsesObject>> {
    let (tuples, errors) = collect(to_response_pair)(map);
    let default = None;
    let object = ResponsesObject::new(tuples, default);
    Ok(Output::new(object, errors))
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
