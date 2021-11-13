use crate::errors::RequestError;
use crate::schemas::{Error, Pet};
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
    OK { content: Pet },
    InternalServerError { content: Error },
}

impl Responder for Response {
    fn to_raw(self) -> HttpResponse {
        match self {
            Response::OK { content } => {
                let mut response = HttpResponse::Ok();
                response.json(content)
            }
            Response::InternalServerError { content } => {
                let mut response = HttpResponse::InternalServerError();
                response.json(content)
            }
        }
    }
}
