use actix_web::{App, HttpServer};
use sample::Handlers;
use sample_server::register_services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().data(Handlers::new());
        register_services!(app, generated in sample)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
