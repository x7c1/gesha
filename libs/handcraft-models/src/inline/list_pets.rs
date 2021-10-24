use crate::inline::RequestError;
use crate::schemas::{Error, Pets};
use actix_web::FromRequest;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub limit: Option<i32>,
}

#[derive(Debug)]
pub struct Request {
    pub query: Query,
    pub raw: HttpRequest,
}

impl Request {
    pub async fn from_raw(raw: HttpRequest) -> Result<Self, RequestError> {
        match web::Query::<Query>::extract(&raw).await {
            Ok(query) => Ok(Request {
                query: query.into_inner(),
                raw,
            }),
            Err(e) => Err(RequestError {
                // TODO:
                key: "list_pets???".to_string(),
                message: e.to_string(),
            }),
        }
    }
}

pub trait Responder {
    fn to_raw(self) -> HttpResponse;
}

#[derive(Debug)]
pub struct ResponseHeaders {
    pub x_next: Option<String>,
}

#[derive(Debug)]
pub enum Response {
    OK {
        headers: ResponseHeaders,
        content: Pets,
    },
    InternalServerError {
        content: Error,
    },
}

impl Responder for Response {
    fn to_raw(self) -> HttpResponse {
        match self {
            Response::OK { headers, content } => {
                let mut response = HttpResponse::Ok();
                if let Some(value) = headers.x_next {
                    response.set_header("x-next", value);
                }
                response.json(content)
            }
            Response::InternalServerError { content } => {
                let mut response = HttpResponse::InternalServerError();
                response.json(content)
            }
        }
    }
}
