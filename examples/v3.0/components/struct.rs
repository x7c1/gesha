pub mod schemas {
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Pet {
        pub id: i64,
        pub name: String,
    }
}
