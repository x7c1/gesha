/*
    Generated by gesha command; DO NOT EDIT BY HAND!
*/
pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Foo(Vec<foo::FooItem>);
    impl From<Vec<foo::FooItem>> for Foo {
        fn from(this: Vec<foo::FooItem>) -> Self {
            Self(this)
        }
    }
    impl From<Foo> for Vec<foo::FooItem> {
        fn from(this: Foo) -> Self {
            this.0
        }
    }

    pub mod foo {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct FooItem {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub bar1: Option<super::Bar1>,
        }
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Bar1(f64);
    impl From<f64> for Bar1 {
        fn from(this: f64) -> Self {
            Self(this)
        }
    }
    impl From<Bar1> for f64 {
        fn from(this: Bar1) -> Self {
            this.0
        }
    }
}
