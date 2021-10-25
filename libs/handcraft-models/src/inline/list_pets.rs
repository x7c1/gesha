use crate::inline::RequestError;
use crate::schemas::{Error, Pets};
use actix_web::{HttpRequest, HttpResponse};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub limit: Option<i32>,
}

impl Query {
    pub async fn from_raw(raw: &HttpRequest) -> Result<Self, RequestError> {
        let kvs: HashMap<String, String> =
            serde_urlencoded::from_str(raw.query_string()).map_err(|e| RequestError {
                key: "query".to_string(),
                message: e.to_string(),
            })?;

        let limit = match kvs.get("limit") {
            Some(value) => Some(value.parse::<i32>().map_err(|e| RequestError {
                key: "limit".to_string(),
                message: e.to_string(),
            })?),
            None => None,
        };
        Ok(Query { limit })
    }
}

#[derive(Debug)]
pub struct Request {
    pub query: Query,
    pub raw: HttpRequest,
}

impl Request {
    pub async fn from_raw(raw: HttpRequest) -> Result<Self, RequestError> {
        let query = Query::from_raw(&raw).await?;
        Ok(Request { query, raw })
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
