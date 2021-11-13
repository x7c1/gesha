use crate::core::{group_by_query_key, iter_to_single_result};
use crate::errors::RequestError;
use crate::schemas::{Error, Pets};
use actix_web::{HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Query {
    pub limit: Option<i32>,
    pub tags: Vec<String>,
}

impl Query {
    pub async fn from_raw(raw: &HttpRequest) -> Result<Self, RequestError> {
        from_query_string(raw.query_string())
    }
}

fn from_query_string(query_string: &str) -> Result<Query, RequestError> {
    let kvs = group_by_query_key(query_string)?;

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

    let value_of_tags = kvs
        .get("tags")
        .map(|values| {
            let iter = values.iter().map(|value| {
                value
                    .parse::<String>()
                    .map_err(|e| RequestError::InvalidQueryValue {
                        key: "tags".to_string(),
                        message: e.to_string(),
                    })
            });
            iter_to_single_result(iter)
        })
        .transpose()?
        .unwrap_or_else(|| vec![]);

    Ok(Query {
        limit: value_of_limit,
        tags: value_of_tags,
    })
}

#[cfg(test)]
mod tests {
    use crate::errors::RequestError;
    use crate::inline::find_pets::{from_query_string, Query};

    #[test]
    fn test_from_query_string() {
        let result = from_query_string("").unwrap();
        assert_eq!(
            result,
            Query {
                limit: None,
                tags: vec![]
            }
        )
    }

    #[test]
    fn test_from_query_string_1() {
        let result = from_query_string("limit=1").unwrap();
        assert_eq!(
            result,
            Query {
                limit: Some(1),
                tags: vec![]
            }
        )
    }

    #[test]
    fn test_from_query_string_2() {
        let result = from_query_string("limit=1&limit=2").unwrap();
        assert_eq!(
            result,
            Query {
                limit: Some(1),
                tags: vec![]
            }
        )
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_from_query_string__empty() {
        let result = from_query_string("limit=").unwrap_err();
        assert_eq!(
            result,
            RequestError::InvalidQueryValue {
                key: "limit".to_string(),
                message: "cannot parse integer from empty string".to_string()
            }
        )
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_from_query_string__multiple() {
        let result = from_query_string("limit=1&limit=2&tags=t1").unwrap();
        assert_eq!(
            result,
            Query {
                limit: Some(1),
                tags: vec!["t1".to_string()]
            }
        )
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_from_query_string__multiple_2() {
        let result = from_query_string("limit=1&limit=2&tags=t1&tags=t2").unwrap();
        assert_eq!(
            result,
            Query {
                limit: Some(1),
                tags: vec!["t1".to_string(), "t2".to_string()]
            }
        )
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_from_query_string__multiple_0() {
        let result = from_query_string("limit=1&limit=2&tags=").unwrap();
        assert_eq!(
            result,
            Query {
                limit: Some(1),
                tags: vec!["".to_string()]
            }
        )
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
pub enum Response {
    OK { content: Pets },
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
