use actix_web::HttpRequest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Path {
    pub pet_id: String,
}

#[derive(Debug)]
pub struct Request {
    pub path: Path,
    pub raw: HttpRequest,
}
