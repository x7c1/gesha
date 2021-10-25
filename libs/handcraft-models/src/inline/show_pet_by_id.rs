use crate::inline::RequestError;
use crate::schemas::{Error, Pet};
use actix_web::{HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Path {
    pub pet_id: String,
}

impl Path {
    pub async fn from_raw(raw: &HttpRequest) -> Result<Self, RequestError> {
        let path_with_segment = raw.match_info();
        let pet_id = path_with_segment
            .get("pet_id")
            .ok_or_else(|| RequestError {
                key: "pet_id".to_string(),
                message: "pet_id required".to_string(),
            })?
            .parse::<String>()
            .map_err(|e| RequestError {
                key: "pet_id".to_string(),
                message: e.to_string(),
            })?;

        Ok(Path { pet_id })
    }
}

#[derive(Debug)]
pub struct Request {
    pub path: Path,
    pub raw: HttpRequest,
}

impl Request {
    pub async fn from_raw(raw: HttpRequest) -> Result<Self, RequestError> {
        let path = Path::from_raw(&raw).await?;
        Ok(Request { path, raw })
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
