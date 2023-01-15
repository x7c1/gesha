pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Sample0 {
        pub id: i64,
        pub sample0_nested1: sample0::Sample0Nested1,
    }

    mod sample0 {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub enum Sample0Nested1 {
            #[serde(rename = "x1")]
            X1,
            #[serde(rename = "x2")]
            X2,
        }
    }
}
