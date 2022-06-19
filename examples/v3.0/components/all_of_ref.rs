pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct BasicErrorModel {
        pub detail: ErrorDetail,
        pub code: i64,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct ExtendedErrorModel {
        pub detail: ErrorDetail,
        pub code: i64,
        pub root_cause: String,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct ErrorDetail {
        pub message: String,
    }
}
