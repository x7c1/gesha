/*
    Generated by gesha command; DO NOT EDIT BY HAND!
*/
pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct BasicErrorModel {
        pub message: String,
        pub code: i64,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct ExtendedErrorModel {
        pub message: String,
        pub code: i64,
        pub root_cause: String,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct ExtendedMore {
        pub message: String,
        pub code: i64,
        pub root_cause: String,
        pub foo: Option<i64>,
    }
}
