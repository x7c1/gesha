/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#operationObject
#[derive(Debug)]
pub struct OperationObject {
    pub responses: ResponsesObject,
}

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
}

/// Response Object | Reference Object
#[derive(Debug)]
pub enum ResponseCase {
    Response(ResponseObject),
    Reference(ReferenceObject),
}

#[derive(Debug)]
pub struct ResponseObject {}

#[derive(Debug)]
pub struct ReferenceObject {}

#[derive(Debug)]
pub enum HttpStatusCode {
    // 200
    OK,
}
