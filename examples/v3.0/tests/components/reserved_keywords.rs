use examples_v3_0::components::reserved_keywords::schemas::ReservedKeywords;

#[test]
fn to_json_string() {
    let x1 = ReservedKeywords {
        break_: "break-value".to_string(),
        ref_: "ref-value".to_string(),
        type_: "type-value".to_string(),
    };
    let actual = serde_json::to_string(&x1).unwrap();
    let expected = r#"{
        "break": "break-value",
        "ref": "ref-value",
        "type": "type-value"
    }"#
    .replace(&[' ', '\n'], "");

    assert_eq!(actual, expected)
}
