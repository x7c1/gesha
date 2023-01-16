pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum Pet {
        Dog(Dog),
        Cat(Cat),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Dog {
        pub bark: Option<bool>,
        pub breed: Option<dog::Breed>,
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
        pub age: Option<i64>,
    }
}
