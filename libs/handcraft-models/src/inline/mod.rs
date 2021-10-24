pub mod index;
pub mod list_pets;
pub mod show_pet_by_id;

#[derive(Debug)]
pub struct RequestError {
    pub key: String,
    pub message: String,
}
