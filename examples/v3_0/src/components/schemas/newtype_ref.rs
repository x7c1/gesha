/*
    Generated by gesha command; DO NOT EDIT BY HAND!
*/
pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Target {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<i64>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Bar(Target);
    impl From<Target> for Bar {
        fn from(this: Target) -> Self {
            Self(this)
        }
    }
    impl From<Bar> for Target {
        fn from(this: Bar) -> Self {
            this.0
        }
    }
}
