use actix_web::{App, HttpServer};
use handcraft::Handlers;
use handcraft_server::register_services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().data(Handlers::new());
        register_services! { app --generated=handcraft }
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}