pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Sample {
        pub id: i64,
        pub nested1: sample::Nested1,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Foo {
        pub foo1: String,
        pub foo2: f64,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Bar {
        pub bar1: String,
        pub bar2: f64,
    }

    pub mod sample {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct Nested1 {
            pub nested2: nested1::Nested2,
        }

        pub mod nested1 {
            use serde::Deserialize;
            use serde::Serialize;

            #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            pub struct Nested2 {
                pub foo1: String,
                pub foo2: f64,
                pub bar1: String,
                pub bar2: f64,
            }
        }
    }
}
