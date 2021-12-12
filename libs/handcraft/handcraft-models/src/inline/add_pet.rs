use crate::core::payload_to_bytes;
use crate::errors::RequestError;
use crate::errors::RequestError::InvalidBody;
use crate::schemas::{Error, NewPet, Pet};
use actix_web::web::Payload;
use actix_web::{HttpRequest, HttpResponse};

#[derive(Debug)]
pub struct Request {
    pub raw: HttpRequest,
    pub body: NewPet,
}

impl Request {
    pub async fn from_raw(raw: HttpRequest, body: Payload) -> Result<Self, RequestError> {
        let bytes = payload_to_bytes(body).await?;
        let body: NewPet = serde_json::from_slice(&bytes).map_err(|e| InvalidBody {
            message: e.to_string(),
        })?;
        Ok(Request { raw, body })
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
