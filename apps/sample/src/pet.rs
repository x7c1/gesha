use crate::Handlers;
use sample_models::inline::show_pet_by_id;
use sample_models::schemas::Pet;
use sample_server_derive::define;

impl Handlers {
    #[define(show_pet_by_id)]
    pub async fn show_pet_by_id(&self, req: show_pet_by_id::Request) -> show_pet_by_id::Response {
        println!("request: {:#?}", req);

        // TODO: remove unwrap
        let id = req.path.pet_id.parse().unwrap();
        let pet = Pet {
            id,
            name: "sample_name".to_string(),
            tag: None,
        };
        show_pet_by_id::Response::OK(pet)
    }
}
