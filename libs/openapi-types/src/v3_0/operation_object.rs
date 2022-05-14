/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#operationObject
#[derive(Debug)]
pub struct OperationObject {
    pub responses: ResponsesObject,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#responsesObject
#[derive(Debug)]
pub struct ResponsesObject {
    pub responses: Vec<(HttpStatusCode, ResponseCase)>,
    pub default: Option<ResponseCase>,
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
