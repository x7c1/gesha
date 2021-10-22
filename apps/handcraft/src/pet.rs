use crate::Handlers;
use handcraft_models::inline::{list_pets, show_pet_by_id};
use handcraft_models::schemas::{Error, Pet, Pets};
use handcraft_server_derive::assert_signature;

impl Handlers {
    #[assert_signature]
    pub async fn show_pet_by_id(
        &self,
        req: show_pet_by_id::Request,
    ) -> impl show_pet_by_id::Responder {
        println!("request: {:#?}", req);

        match req.path.pet_id.parse() {
            Ok(id) => show_pet_by_id::Response::OK {
                content: Pet {
                    id,
                    name: "handcraft_name".to_string(),
                    tag: None,
                },
            },
            Err(e) => show_pet_by_id::Response::InternalServerError {
                content: Error {
                    code: 1,
                    message: e.to_string(),
                },
            },
        }
    }
    #[assert_signature]
    pub async fn list_pets(&self, _req: list_pets::Request) -> impl list_pets::Responder {
        list_pets::Response::OK {
            headers: list_pets::ResponseHeaders { x_next: None },
            content: Pets::new(vec![]),
        }
    }
}
