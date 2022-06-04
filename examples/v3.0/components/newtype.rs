pub mod schemas {
    pub struct StringValue(String);

    pub struct IntegerDefaultValue(i64);

    pub struct Int64Value(i64);

    pub struct Int32Value(i32);

    pub struct NumberDefaultValue(f64);

    pub struct FloatValue(f32);

    pub struct DoubleValue(f64);

    pub struct BooleanValue(bool);

    pub struct ArrayValue(Vec<i64>);
}
