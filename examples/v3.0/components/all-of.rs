pub mod schemas {
    pub struct BasicErrorModel {
        pub message: String,
        pub code: i64,
    }

    pub struct ExtendedErrorModel {
        pub message: String,
        pub code: i64,
        pub root_cause: String,
    }
}
