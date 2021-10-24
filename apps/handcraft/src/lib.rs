use handcraft_models::inline::RequestError;
use handcraft_server::BadRequestHandler;
use handcraft_server_derive::Handcraft;

mod index;
mod pet;

#[derive(Handcraft)]
pub struct Handlers {
    foo: String,
}

impl Default for Handlers {
    fn default() -> Self {
        Handlers {
            foo: "fooooo".to_string(),
        }
    }
}

impl BadRequestHandler for Handlers {
    fn on_bad_request(&self, error: RequestError) -> actix_web::HttpResponse {
        println!("on_bad_request...{:#?}", error);
        todo!()
    }
}
