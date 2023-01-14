pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Base {
        pub x1: Option<String>,
        pub x2: Option<i64>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Extended {
        pub x1: String,
        pub x2: Option<i64>,
        pub foo: Option<String>,
    }
}
