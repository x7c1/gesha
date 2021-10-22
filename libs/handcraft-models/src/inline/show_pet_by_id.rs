use crate::schemas::{Error, Pet};
use actix_web::{HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Path {
    pub pet_id: String,
}

#[derive(Debug)]
pub struct Request {
    pub path: Path,
    pub raw: HttpRequest,
}

pub trait Responder {
    fn to_raw(self) -> HttpResponse;
}

#[derive(Debug)]
pub enum Response {
    Ok { content: Pet },
    InternalServerError { content: Error },
}

impl Responder for Response {
    fn to_raw(self) -> HttpResponse {
        match self {
            Response::Ok { content } => {
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
