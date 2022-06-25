use examples_v3_0::components::struct_simple::schemas::Pet;

#[test]
fn to_json_string() {
    let pet = Pet { id: 123, name: "sample_name".to_string() };
    let actual = serde_json::to_string(&pet).unwrap();
    assert_eq!(actual, r#"{"id":123,"name":"sample_name"}"#)
}

#[test]
fn from_json_string() {
    let actual = serde_json::from_str::<Pet>(r#"{"id":111,"name":"sample"}"#).unwrap();
    assert_eq!(actual, Pet { id: 111, name: "sample".to_string() })
}
