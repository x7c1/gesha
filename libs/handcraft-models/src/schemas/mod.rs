use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pet {
    pub id: i64,
    pub name: String,
    pub tag: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Pets(Vec<Pet>);

impl Pets {
    pub fn new(xs: Vec<Pet>) -> Self {
        Pets(xs)
    }
}
