pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    /**
    https://swagger.io/docs/specification/data-models/oneof-anyof-allof-not/
    */
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum Pet {
        Cat(Cat),
        Dog(Dog),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Dog {
        pub bark: Option<bool>,
        pub breed: dog::Breed,
    }

    pub mod dog {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub enum Breed {
            Dingo,
            Husky,
            Retriever,
            Shepherd,
        }
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Cat {
        pub hunts: Option<bool>,
        pub age: i64,
    }
}
