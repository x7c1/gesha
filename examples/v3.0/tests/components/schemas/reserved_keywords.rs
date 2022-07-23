use crate::components::flatten;
use examples_v3_0::components::schemas::reserved_keywords::schemas::ReservedKeywords;

#[test]
fn to_json_string() {
    let x1 = ReservedKeywords {
        break_: "break-value".to_string(),
        continue_: "continue-value".to_string(),
        move_: "move-value".to_string(),
        ref_: "ref-value".to_string(),
        type_: "type-value".to_string(),
    };
    let actual = serde_json::to_string(&x1).unwrap();
    let expected = flatten(
        r#"{
            "break": "break-value",
            "continue": "continue-value",
            "move": "move-value",
            "ref": "ref-value",
            "type": "type-value"
        }"#,
    );

    assert_eq!(actual, expected)
}
