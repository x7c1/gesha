use crate::errors::RequestError;
use crate::schemas::Error;
use actix_web::{HttpRequest, HttpResponse};

#[derive(Debug)]
pub struct Request {
    pub raw: HttpRequest,
}

impl Request {
    pub async fn from_raw(raw: HttpRequest) -> Result<Self, RequestError> {
        Ok(Request { raw })
    }
}

pub trait Responder {
    fn to_raw(self) -> HttpResponse;
}

#[derive(Debug)]
pub enum Response {
    Created,
    InternalServerError { content: Error },
}

impl Responder for Response {
    fn to_raw(self) -> HttpResponse {
        match self {
            Response::Created => {
                let mut response = HttpResponse::Created();
                response.body(())
            }
            Response::InternalServerError { content } => {
                let mut response = HttpResponse::InternalServerError();
                response.json(content)
            }
        }
    }
}
