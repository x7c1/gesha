use crate::core::OutputOptionOps;
use crate::error::by_key;
use crate::v3_0::yaml_extractor::collect;
use crate::v3_0::{
    HttpStatusCode, OperationObject, ResponseCase, ResponseObject, ResponsesObject, YamlExtractor,
};
use crate::yaml::YamlMap;
use crate::{Error, Output, Result, with_key};

#[allow(dead_code)]
#[derive(Debug)]
/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#pathsObject
pub struct PathsObject(Vec<(PathFieldName, PathItemObject)>);

impl PathsObject {
    /// > The Paths MAY be empty, due to ACL constraints.
    pub fn new(paths: Vec<(PathFieldName, PathItemObject)>) -> Self {
        // TODO: check if each PathFieldName is unique in paths
        PathsObject(paths)
    }

    pub fn from_yaml_map(map: YamlMap) -> Output<PathsObject> {
        collect(Output::by(to_path_pair))(map).map(PathsObject::new)
    }
}

/// e.g. /pets
#[allow(dead_code)]
#[derive(Debug)]
pub struct PathFieldName(String);

impl PathFieldName {
    /// > The field name MUST begin with a forward slash (/).
    pub fn new<A: Into<String>>(a: A) -> Self {
        // TODO: check field pattern
        PathFieldName(a.into())
    }
}

#[derive(Debug)]
pub struct PathItemObject {
    pub get: Option<OperationObject>,
    pub post: Option<OperationObject>,
}

fn to_path_pair(kv: (String, YamlMap)) -> Result<(PathFieldName, PathItemObject)> {
    let (field, map) = kv;
    let pair = (
        PathFieldName::new(&field),
        to_path_item_object(map).map_err(by_key(field))?,
    );
    Ok(pair)
}

fn to_path_item_object(mut map: YamlMap) -> Result<PathItemObject> {
    let (get, get_errors) = map
        .try_extract_if_exists("get", to_operation_object)
        .map(|x| x.maybe())
        .flatten()
        .into_tuple();

    let (post, post_errors) = map
        .try_extract_if_exists("post", to_operation_object)
        .map(|x| x.maybe())
        .flatten()
        .into_tuple();

    let object = PathItemObject { get, post };
    let output = Output::ok(object).append(get_errors).append(post_errors);
    output.to_result().map_err(Error::multiple)
}

fn to_operation_object(mut map: YamlMap) -> Result<Output<OperationObject>> {
    let responses = map.extract("responses")?;
    let (responses, errors) = to_responses_object(responses)
        .bind_errors(with_key("responses"))
        .into_tuple();

    let object = OperationObject { responses };
    Ok(Output::new(object, errors))
}

fn to_responses_object(map: YamlMap) -> Output<ResponsesObject> {
    let (tuples, errors) = collect(to_response_pair)(map).into_tuple();
    let default = None;
    let object = ResponsesObject::new(tuples, default);
    Output::new(object, errors)
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
    Ok(Output::ok(case))
}
