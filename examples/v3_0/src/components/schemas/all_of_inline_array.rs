/*
    Generated by gesha command; DO NOT EDIT BY HAND!
*/
pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct SampleList(Vec<sample_list::SampleListItem>);
    impl From<Vec<sample_list::SampleListItem>> for SampleList {
        fn from(this: Vec<sample_list::SampleListItem>) -> Self {
            Self(this)
        }
    }
    impl From<SampleList> for Vec<sample_list::SampleListItem> {
        fn from(this: SampleList) -> Self {
            this.0
        }
    }

    pub mod sample_list {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct SampleListItem {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub x1: Option<sample_list_item::X1>,
        }

        pub mod sample_list_item {
            use serde::Deserialize;
            use serde::Serialize;

            #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            pub struct X1 {
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub foo1: Option<String>,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub bar1: Option<String>,
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Foo {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub foo1: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Bar {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bar1: Option<String>,
    }
}
