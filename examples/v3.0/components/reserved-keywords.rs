pub mod schemas {
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Pet {
        pub type_: String,
    }
}
