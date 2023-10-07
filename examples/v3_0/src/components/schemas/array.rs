/*
    Generated by gesha command; DO NOT EDIT BY HAND!
*/
pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Pet {
        pub id: i64,
        pub string_values: Vec<String>,
        pub int32_values: Vec<i32>,
        pub array_array: Vec<Vec<i64>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub not_required: Option<Vec<i64>>,
    }
}
