/*
    Generated by gesha command; DO NOT EDIT BY HAND!
*/
pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Foo {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub foo1: Option<foo::Foo1>,
    }

    pub mod foo {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct Foo1 {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub foo2: Option<String>,

            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub foo3: Option<super::Bar>,
        }
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Bar {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bar1: Option<String>,
    }
}
