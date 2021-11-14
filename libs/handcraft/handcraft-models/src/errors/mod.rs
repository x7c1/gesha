use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(tag = "type", content = "content")]
pub enum RequestError {
    QueryStringBroken(String),
    InvalidQueryValue { key: String, message: String },
    InvalidPathValue { key: String, message: String },
    InvalidBody { message: String },
    EmptyPathValue { key: String },
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
            .replace(" ", "")
            .replace("\n", "")
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
            .replace(" ", "")
            .replace("\n", "")
        );
    }
}
