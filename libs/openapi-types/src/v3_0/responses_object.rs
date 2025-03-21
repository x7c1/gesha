use crate::Output;
use crate::v3_0::ReferenceObject;
use crate::v3_0::yaml_extractor::collect;
use crate::yaml::YamlMap;

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#responsesObject
#[derive(Debug)]
pub struct ResponsesObject {
    _responses: Vec<(HttpStatusCode, ResponseCase)>,
    _default: Option<ResponseCase>,
}

impl ResponsesObject {
    /// > The Responses Object MUST contain at least one response code,
    /// > and it SHOULD be the response for a successful operation call.
    pub fn new(
        responses: Vec<(HttpStatusCode, ResponseCase)>,
        default: Option<ResponseCase>,
    ) -> Self {
        // TODO: check if arguments satisfy specifications
        ResponsesObject {
            _responses: responses,
            _default: default,
        }
    }

    pub fn from_yaml_map(map: YamlMap) -> Output<Self> {
        let (tuples, errors) = collect(to_response_pair)(map).into_tuple();
        let default = None;
        let object = ResponsesObject::new(tuples, default);
        Output::new(object, errors)
    }
}

/// Response Object | Reference Object
#[derive(Debug)]
pub enum ResponseCase {
    Response(ResponseObject),
    Reference(ReferenceObject<ResponseObject>),
}

#[derive(Debug)]
pub struct ResponseObject {}

#[derive(Debug)]
pub enum HttpStatusCode {
    // 200
    OK,
}

fn to_response_pair(
    kv: (String, YamlMap),
) -> crate::Result<Output<(HttpStatusCode, ResponseCase)>> {
    let (field, map) = kv;
    let code = to_http_status_code(field)?;
    let output = to_response_case(map)?.map(|case| (code, case));
    Ok(output)
}

fn to_http_status_code(_v: String) -> crate::Result<HttpStatusCode> {
    // TODO:
    Ok(HttpStatusCode::OK)
}

fn to_response_case(_map: YamlMap) -> crate::Result<Output<ResponseCase>> {
    // TODO:
    let case = ResponseCase::Response(ResponseObject {});
    Ok(Output::ok(case))
}
