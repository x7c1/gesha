use examples_v3_0::components::enums::schemas::StringEnum1;

#[test]
fn to_json_string() {
    let x1 = StringEnum1::Error1;
    let actual = serde_json::to_string(&x1).unwrap();
    assert_eq!(actual, r#""ERROR1""#)
}
