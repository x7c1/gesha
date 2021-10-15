mod index;

use actix_web::{App, HttpServer};
use sample_server::register_services;
use sample_server_derive::Sample;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().data(Handlers::new());
        register_services!(app)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(Sample)]
pub struct Handlers {
    foo: String,
}

impl Handlers {
    fn new() -> Self {
        Handlers {
            foo: "fooooo".to_string(),
        }
    }
}
