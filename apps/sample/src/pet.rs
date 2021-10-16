use crate::Handlers;
use sample_models::inline::show_pet_by_id;
use sample_models::schemas::Pet;
use sample_server_derive::define;

impl Handlers {
    #[define(show_pet_by_id)]
    pub async fn show_pet_by_id(&self, req: show_pet_by_id::Request) -> Pet {
        println!("request: {:#?}", req);
        Pet {
            id: 0,
            name: "sample_name".to_string(),
            tag: None,
        }
    }
}
