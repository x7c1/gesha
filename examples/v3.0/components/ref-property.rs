pub mod schemas {
    pub struct PetContainer {
        pub pet: Pet,
    }
    pub struct Pet {
        pub id: i64,
        pub name: String,
    }
}
