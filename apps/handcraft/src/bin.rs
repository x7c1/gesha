use actix_web::web::Data;
use actix_web::{App, HttpServer};
use handcraft::Handlers;
use handcraft::multipart::post_multipart_form_data;
use handcraft_server::register_services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .app_data(Data::new(Handlers::default()))
            .service(post_multipart_form_data);

        register_services! { app --generated=handcraft }
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
