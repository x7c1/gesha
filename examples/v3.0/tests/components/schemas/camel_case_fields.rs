use crate::components::flatten;
use examples_v3_0::components::schemas::camel_case_fields::schemas::CamelCaseFields;

#[test]
fn to_json_string() {
    let actual = serde_json::to_string(&CamelCaseFields {
        lower_camel_case: "foo".to_string(),
        upper_camel_case: "bar".to_string(),
    })
    .unwrap();

    let expected = flatten(
        r#"{
            "lowerCamelCase": "foo",
            "UpperCamelCase": "bar"
        }"#,
    );
    assert_eq!(actual, expected)
}

#[test]
fn from_json_string() {
    let actual = serde_json::from_str::<CamelCaseFields>(
        r#"{
            "lowerCamelCase": "foo",
            "UpperCamelCase": "bar"
        }"#,
    )
    .unwrap();

    let expected = CamelCaseFields {
        lower_camel_case: "foo".to_string(),
        upper_camel_case: "bar".to_string(),
    };
    assert_eq!(actual, expected)
}
