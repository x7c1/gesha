pub mod schemas {
    pub struct StringValue(String);

    pub struct BooleanValue(bool);

    pub struct ArrayValue(Vec<i64>);

    pub struct ArrayCustomValue(Vec<Pet>);

    pub struct Pet {
        pub id: i64,
    }
}
