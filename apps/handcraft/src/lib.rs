use actix_web::HttpResponse;
use handcraft_models::errors::RequestError;
use handcraft_models::schemas;
use handcraft_server::BadRequestHandler;
use handcraft_server_derive::Handcraft;

mod index;
mod petstore;

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
        HttpResponse::BadRequest().json(from_request_error(error))
    }
}

fn from_request_error(e: RequestError) -> schemas::Error {
    let (code, message) = match e {
        RequestError::QueryStringBroken(s) => (4001, s),
        RequestError::InvalidQueryValue { key, message } => {
            (4002, format!("{} [query-key:{}]", message, key))
        }
        RequestError::InvalidPathValue { key, message } => {
            (4003, format!("{} [path-key:{}]", message, key))
        }
        RequestError::EmptyPathValue { key } => (4004, format!("[key:{}] required.", key)),
        RequestError::InvalidBody { message } => (4005, message),
    };
    schemas::Error { code, message }
}
