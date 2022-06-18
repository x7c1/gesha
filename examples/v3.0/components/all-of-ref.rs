pub mod schemas {
    pub struct BasicErrorModel {
        pub detail: ErrorDetail,
        pub code: i64,
    }

    pub struct ExtendedErrorModel {
        pub detail: ErrorDetail,
        pub code: i64,
        pub root_cause: String,
    }

    pub struct ErrorDetail {
        pub message: String,
    }
}
