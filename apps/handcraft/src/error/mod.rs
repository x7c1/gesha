use actix_web::http::StatusCode;
use actix_web::ResponseError;
use handcraft_models::errors::RequestError;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum ApiError {
    InvalidRequest,
}

impl From<RequestError> for ApiError {
    fn from(raw: RequestError) -> Self {
        // TODO:
        match raw {
            RequestError::QueryStringBroken(_) => {}
            RequestError::InvalidQueryValue { .. } => {}
            RequestError::InvalidPathValue { .. } => {}
            RequestError::InvalidBody { .. } => {}
            RequestError::EmptyPathValue { .. } => {}
            RequestError::FormDataFieldRequired { .. } => {}
            RequestError::MultipartError { .. } => {}
            RequestError::ContentDispositionNotFound => {}
            RequestError::ContentDispositionNameNotFound => {}
            RequestError::JsonFormatError { .. } => {}
        }
        ApiError::InvalidRequest
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO: impl Display for SampleError")
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InvalidRequest => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        todo!()
    }
}
