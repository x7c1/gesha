pub mod schemas {
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct IntegerDefaultValue(i64);

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Int64Value(i64);

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Int32Value(i32);

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct NumberDefaultValue(f64);

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct FloatValue(f32);

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct DoubleValue(f64);
}
