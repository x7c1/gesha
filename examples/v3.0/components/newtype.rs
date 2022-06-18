pub mod schemas {
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct StringValue(String);

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct BooleanValue(bool);

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct ArrayValue(Vec<i64>);

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct ArrayCustomValue(Vec<Pet>);

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Pet {
        pub id: i64,
    }
}
