use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Pet {
    pub id: i64,
    pub name: String,
    pub tag: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Pets(pub Vec<Pet>);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NewPet {
    pub name: String,
    pub tag: Option<String>,
}
