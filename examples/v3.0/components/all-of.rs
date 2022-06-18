pub mod schemas {
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
}
