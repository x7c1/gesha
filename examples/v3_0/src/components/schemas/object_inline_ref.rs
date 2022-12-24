pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct SamplePet {
        pub id: i64,
        pub nested1: sample_pet::Nested1,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Coordinate {
        pub latitude: f64,
        pub longitude: f64,
    }

    pub mod sample_pet {
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
                pub location: super::super::Coordinate,
            }
        }
    }
}
