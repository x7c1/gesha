pub mod schemas {
    pub struct PetsContainer {
        pub pets: Vec<Pet>,
    }

    pub struct Pet {
        pub id: i64,
        pub name: String,
    }
}
