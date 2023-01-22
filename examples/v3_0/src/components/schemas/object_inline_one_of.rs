pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Sample1 {
        pub id: i64,
        pub sample1_nested1: sample1::Sample1Nested1,
    }

    pub mod sample1 {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct Sample1Nested1 {
            pub sample1_nested2: sample1_nested1::Sample1Nested2,
        }

        pub mod sample1_nested1 {
            use serde::Deserialize;
            use serde::Serialize;

            #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            #[serde(untagged)]
            pub enum Sample1Nested2 {
                Foo(super::super::Foo),
                Bar(super::super::Bar),
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Foo {
        pub foo1: String,
        pub foo2: Option<f64>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Bar {
        pub bar1: String,
        pub bar2: Option<f64>,
    }
}
