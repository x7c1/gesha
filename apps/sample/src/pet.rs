use crate::Handlers;
use sample_models::inline::show_pet_by_id;
use sample_models::schemas::{Error, Pet};
use sample_server_derive::assert_signature;

impl Handlers {
    #[assert_signature]
    pub async fn show_pet_by_id(&self, req: show_pet_by_id::Request) -> show_pet_by_id::Response {
        println!("request: {:#?}", req);

        match req.path.pet_id.parse() {
            Ok(id) => show_pet_by_id::Response::OK(Pet {
                id,
                name: "sample_name".to_string(),
                tag: None,
            }),
            Err(e) => show_pet_by_id::Response::InternalServerError(Error {
                code: 1,
                message: e.to_string(),
            }),
        }
    }
}