/*
    Generated by gesha command; DO NOT EDIT BY HAND!
*/
pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct IntegerDefaultValue(i64);
    impl From<i64> for IntegerDefaultValue {
        fn from(a: i64) -> IntegerDefaultValue {
            Self(a)
        }
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Int64Value(i64);
    impl From<i64> for Int64Value {
        fn from(a: i64) -> Int64Value {
            Self(a)
        }
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Int32Value(i32);
    impl From<i32> for Int32Value {
        fn from(a: i32) -> Int32Value {
            Self(a)
        }
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct NumberDefaultValue(f64);
    impl From<f64> for NumberDefaultValue {
        fn from(a: f64) -> NumberDefaultValue {
            Self(a)
        }
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct FloatValue(f32);
    impl From<f32> for FloatValue {
        fn from(a: f32) -> FloatValue {
            Self(a)
        }
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct DoubleValue(f64);
    impl From<f64> for DoubleValue {
        fn from(a: f64) -> DoubleValue {
            Self(a)
        }
    }
}
