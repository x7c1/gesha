/*
    Generated by gesha command; DO NOT EDIT BY HAND!
*/
pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Pet {
        pub id: i64,
        #[serde(rename = "lowerCamelCase")]
        pub lower_camel_case: String,
        #[serde(rename = "UpperCamelCase")]
        pub upper_camel_case: String,
    }
}
