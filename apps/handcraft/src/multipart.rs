use actix_web::http::StatusCode;
use actix_web::{post, HttpRequest, ResponseError};
use handcraft_models::errors::RequestError;
use handcraft_models::inline::post_multipart_form_data::Response::Created;
use handcraft_models::inline::post_multipart_form_data::{Request, Response};
use handcraft_models::schemas::MultiPartFormDataResponse;
use std::fmt::{Debug, Display, Formatter};

#[post("/multipart_form_data")]
pub async fn post_multipart_form_data(request: Request) -> Result<Response, SampleError> {
    println!("[start] post_multipart_form_data: {:#?}", request.body?);

    Ok(Created {
        content: MultiPartFormDataResponse {},
    })
}

#[derive(Debug)]
pub enum SampleError {
    InvalidRequest,
}

impl From<RequestError> for SampleError {
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
        }
        SampleError::InvalidRequest
    }
}

impl Display for SampleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO: impl Display for SampleError")
    }
}

impl ResponseError for SampleError {
    fn status_code(&self) -> StatusCode {
        match self {
            SampleError::InvalidRequest => StatusCode::BAD_REQUEST,
        }
    }
}
