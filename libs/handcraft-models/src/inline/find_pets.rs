use crate::errors::RequestError;
use crate::schemas::{Error, Pets};
use actix_web::{HttpRequest, HttpResponse};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Query {
    pub limit: Option<i32>,
    pub tags: Option<Vec<String>>,
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
            let iter = values.iter().map(|s| {
                s.parse::<String>()
                    .map_err(|e| RequestError::InvalidQueryValue {
                        key: "tags".to_string(),
                        message: e.to_string(),
                    })
            });
            iter_to_single_result(iter)
        })
        .transpose()?;

    Ok(Query {
        limit: value_of_limit,
        tags: value_of_tags,
    })
}

fn group_by_query_key(query_string: &str) -> Result<HashMap<String, Vec<String>>, RequestError> {
    let pairs: Vec<(String, String)> = serde_urlencoded::from_str(query_string)
        .map_err(|e| RequestError::QueryStringBroken(e.to_string()))?;

    let mut kvs = HashMap::<String, Vec<String>>::new();
    for (k, v) in pairs {
        kvs.entry(k).or_insert(vec![]).push(v)
    }

    Ok(kvs)
}

fn iter_to_single_result<A, B>(xs: impl Iterator<Item = Result<A, B>>) -> Result<Vec<A>, B> {
    let mut ys = vec![];
    for x in xs {
        match x {
            Ok(value) => ys.push(value),
            Err(e) => return Err(e),
        }
    }
    Ok(ys)
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
                tags: None
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
                tags: None
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
                tags: None
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
                tags: Some(vec!["t1".to_string()])
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
