use crate::core::{OutputOptionOps, OutputPairOps};
use crate::v3_0::{
    HttpStatusCode, OperationObject, PathFieldName, PathItemObject, PathsObject, ResponseCase,
    ResponseObject, ResponsesObject,
};
use crate::yaml::{collect, YamlMap};
use crate::{with_key, Output, Result};

pub(super) fn to_paths_object(map: YamlMap) -> Result<Output<PathsObject>> {
    let output = collect(to_path_pair)(map).map(PathsObject::new);
    Ok(output)
}

fn to_path_pair(kv: (String, YamlMap)) -> Result<Output<(PathFieldName, PathItemObject)>> {
    let (field, map) = kv;
    let pair = (
        PathFieldName::new(&field),
        to_path_item_object(map)?.map_errors(with_key(field)),
    );
    Ok(pair.lift())
}

fn to_path_item_object(mut map: YamlMap) -> Result<Output<PathItemObject>> {
    let (get, get_errors) = map
        .remove_if_exists("get")?
        .map(to_operation_object)
        .transpose()?
        .maybe()
        .map_errors(with_key("get"))
        .to_tuple();

    let (post, post_errors) = map
        .remove_if_exists("post")?
        .map(to_operation_object)
        .transpose()?
        .maybe()
        .map_errors(with_key("post"))
        .to_tuple();

    let object = PathItemObject { get, post };
    let output = Output::new(object, get_errors).append(post_errors);
    Ok(output)
}

fn to_operation_object(mut map: YamlMap) -> Result<Output<OperationObject>> {
    let responses = map.remove("responses")?;
    let (responses, errors) = to_responses_object(responses)?
        .map_errors(with_key("responses"))
        .to_tuple();

    let object = OperationObject { responses };
    Ok(Output::new(object, errors))
}

fn to_responses_object(map: YamlMap) -> Result<Output<ResponsesObject>> {
    let (tuples, errors) = collect(to_response_pair)(map).to_tuple();
    let default = None;
    let object = ResponsesObject::new(tuples, default);
    Ok(Output::new(object, errors))
}

fn to_response_pair(kv: (String, YamlMap)) -> Result<Output<(HttpStatusCode, ResponseCase)>> {
    let (field, map) = kv;
    let code = to_http_status_code(field)?;
    let output = to_response_case(map)?.map(|case| (code, case));
    Ok(output)
}

fn to_http_status_code(_v: String) -> Result<HttpStatusCode> {
    // TODO:
    Ok(HttpStatusCode::OK)
}

fn to_response_case(_map: YamlMap) -> Result<Output<ResponseCase>> {
    // TODO:
    let case = ResponseCase::Response(ResponseObject {});
    Ok(Output::new(case, vec![]))
}
