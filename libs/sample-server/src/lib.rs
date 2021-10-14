use actix_web::HttpRequest;
use serde::Deserialize;

#[macro_export]
macro_rules! sample_server {
    ($api: expr) => {
        actix_web::HttpServer::new(|| actix_web::App::new().data($api).service(generated::index))
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
