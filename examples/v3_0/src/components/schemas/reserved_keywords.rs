/*
    Generated by gesha command; DO NOT EDIT BY HAND!
*/
pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct ReservedKeywords {
        #[serde(rename = "break")]
        pub break_: String,

        #[serde(rename = "continue")]
        pub continue_: String,

        #[serde(rename = "move")]
        pub move_: String,

        #[serde(rename = "ref")]
        pub ref_: String,

        #[serde(rename = "type")]
        pub type_: String,
    }
}
