use serde_json::Value;
use examples_v3_0::components::newtype::schemas::{BooleanValue, StringValue};

#[test]
fn to_json_string() {
    let x1 = StringValue::from("x1".to_string());
    let actual = serde_json::to_value(&x1).unwrap();
    assert_eq!(actual, Value::String("x1".to_string()))
}

#[test]
fn to_json_bool() {
    let x1 = BooleanValue::from(true);
    let actual = serde_json::to_value(&x1).unwrap();
    assert_eq!(actual, Value::Bool(true))
}

#[test]
fn into_string() {
    let x1 = "x1".to_string();
    let typed = StringValue::from(x1);
    let actual: String = typed.into();
    assert_eq!(actual, "x1")
}
