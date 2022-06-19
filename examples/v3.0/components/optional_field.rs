pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Pet {
        pub id: i64,
        pub tag: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct OptionalRef {
        pub pet: Option<Pet>,
    }
}