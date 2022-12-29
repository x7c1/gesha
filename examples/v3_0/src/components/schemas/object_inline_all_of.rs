pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Sample0 {
        pub id: i64,
        pub sample0_nested1: sample0::Sample0Nested1,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Sample1 {
        pub id: i64,
        pub sample1_nested1: sample1::Sample1Nested1,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Sample2 {
        pub id: i64,
        pub sample2_nested1: sample2::Sample2Nested1,
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

    pub mod sample0 {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct Sample0Nested1 {
            pub foo1: String,
            pub foo2: f64,
            pub bar1: String,
            pub bar2: f64,
        }
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
            pub struct Sample1Nested2 {
                pub foo1: String,
                pub foo2: f64,
                pub bar1: String,
                pub bar2: f64,
            }
        }
    }

    pub mod sample2 {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct Sample2Nested1 {
            pub sample2_nested2: sample2_nested1::Sample2Nested2,
        }

        pub mod sample2_nested1 {
            use serde::Deserialize;
            use serde::Serialize;

            #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            pub struct Sample2Nested2 {
                pub foo1: String,
                pub foo2: f64,
                pub sample2_nested3: sample2_nested2::Sample2Nested3,
            }

            pub mod sample2_nested2 {
                use serde::Deserialize;
                use serde::Serialize;

                #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
                pub struct Sample2Nested3 {
                    pub bar: super::super::super::Bar,
                }
            }
        }
    }
}
