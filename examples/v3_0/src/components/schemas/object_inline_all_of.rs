pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Sample0 {
        pub id: i64,
        pub nested1: sample0::Nested1,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Sample1 {
        pub id: i64,
        pub nested1: sample1::Nested1,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Sample2 {
        pub id: i64,
        pub nested1: sample2::Nested1,
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
        pub struct Nested1 {
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

    pub mod sample2 {
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
                pub nested3: nested2::Nested3,
            }

            pub mod nested2 {
                use serde::Deserialize;
                use serde::Serialize;

                #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
                pub struct Nested3 {
                    pub bar: super::super::super::Bar,
                }
            }
        }
    }
}
