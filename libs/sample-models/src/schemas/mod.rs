use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Pet {
    pub id: i64,
    pub name: String,
    pub tag: Option<String>,
}
