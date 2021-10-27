use actix_web::HttpResponse;
use handcraft_models::errors::RequestError;
use handcraft_models::schemas;
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
    fn on_bad_request(&self, error: RequestError) -> HttpResponse {
        // sample codes mapping RequestError to schemas::Error.
        HttpResponse::BadRequest().json(to_api_error(error))
    }
}

fn to_api_error(e: RequestError) -> schemas::Error {
    let (code, message) = match e {
        RequestError::QueryStringBroken(s) => (4001, s),
        RequestError::InvalidQueryValue { key, message } => {
            (4002, format!("{} [query-key:{}]", message, key))
        }
        RequestError::InvalidPathValue { key, message } => {
            (4003, format!("{} [path-key:{}]", message, key))
        }
        RequestError::EmptyPathValue { key } => (4004, format!("[key:{}] required.", key)),
    };
    schemas::Error { code, message }
}
