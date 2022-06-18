pub mod schemas {
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Pet {
        pub id: i64,
        pub string_values: Vec<String>,
        pub int32_values: Vec<i32>,
        pub array_array: Vec<Vec<i64>>,
    }
}
