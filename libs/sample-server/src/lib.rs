use actix_web::HttpRequest;
use serde::Deserialize;

#[macro_export]
macro_rules! register_services {
    ($app: ident) => {
        $app.service(generated::index)
    };
}

#[macro_export]
macro_rules! http_server {
    ($handlers: expr) => {
        actix_web::HttpServer::new(|| {
            let app = actix_web::App::new().data($handlers);
            register_services!(app)
        })
    };
}

#[derive(Debug, Deserialize)]
pub struct IndexPath {
    pub id: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct IndexRequest {
    pub path: IndexPath,
    pub raw: HttpRequest,
}
