use actix_multipart::MultipartError;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(tag = "type", content = "content")]
pub enum RequestError {
    QueryStringBroken(String),
    InvalidQueryValue { key: String, message: String },
    InvalidPathValue { key: String, message: String },
    InvalidBody { message: String },
    EmptyPathValue { key: String },
    JsonFormatError { key: String, message: String },
    // multipart/form-data
    FormDataFieldRequired { name: String },
    MultipartError { cause: String },
    ContentDispositionNotFound,
    ContentDispositionNameNotFound,
}

impl From<MultipartError> for RequestError {
    fn from(cause: MultipartError) -> Self {
        Self::MultipartError {
            cause: format!("{:#?}", cause),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::RequestError;

    #[test]
    fn query_string_broken() {
        let e = RequestError::QueryStringBroken("abcde".to_string());
        assert_eq!(
            serde_json::to_string(&e).unwrap(),
            r#"{
                "type": "QueryStringBroken",
                "content": "abcde"
            }"#
            .replace([' ', '\n'], "")
        );
    }

    #[test]
    fn invalid_query_value() {
        let e = RequestError::InvalidQueryValue {
            key: "k1".to_string(),
            message: "m1".to_string(),
        };
        assert_eq!(
            serde_json::to_string(&e).unwrap(),
            r#"{
                "type": "InvalidQueryValue",
                "content": {
                    "key": "k1",
                    "message": "m1"
                }
            }"#
            .replace([' ', '\n'], "")
        );
    }
}
