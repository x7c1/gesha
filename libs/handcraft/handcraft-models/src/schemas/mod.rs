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

impl From<Pet> for NewPet {
    fn from(pet: Pet) -> Self {
        NewPet {
            name: pet.name,
            tag: pet.tag,
        }
    }
}

impl NewPetLike for Pet {
    fn name(&self) -> &str {
        &self.name
    }

    fn tag(&self) -> &Option<String> {
        &self.tag
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Pets(pub Vec<Pet>);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NewPet {
    pub name: String,
    pub tag: Option<String>,
}

trait NewPetLike {
    fn name(&self) -> &str;
    fn tag(&self) -> &Option<String>;
}

impl NewPetLike for NewPet {
    fn name(&self) -> &str {
        &self.name
    }

    fn tag(&self) -> &Option<String> {
        &self.tag
    }
}
