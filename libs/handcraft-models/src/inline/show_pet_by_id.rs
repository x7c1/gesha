use crate::inline::RequestError;
use crate::schemas::{Error, Pet};
use actix_web::{web, HttpRequest, HttpResponse};
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

impl Request {
    pub async fn from_raw(raw: HttpRequest) -> Result<Self, RequestError> {
        use actix_web::FromRequest;
        let x = web::Path::<Path>::extract(&raw).await;
        // TODO:
        let path = x.unwrap();
        Ok(Request {
            path: path.into_inner(),
            raw,
        })
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
