use crate::core::group_by_query_key;
use crate::errors::RequestError;
use crate::schemas::{Error, Pets};
use actix_web::{HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub limit: Option<i32>,
}

impl Query {
    pub async fn from_raw(raw: &HttpRequest) -> Result<Self, RequestError> {
        let kvs = group_by_query_key(raw.query_string())?;

        let value_of_limit = kvs
            .get("limit")
            .map(|values| {
                values[0]
                    .parse::<i32>()
                    .map_err(|e| RequestError::InvalidQueryValue {
                        key: "limit".to_string(),
                        message: e.to_string(),
                    })
            })
            .transpose()?;

        Ok(Query {
            limit: value_of_limit,
        })
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
                    response.insert_header(("x-next", value));
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
