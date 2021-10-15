use actix_web::HttpRequest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Path {
    pub id: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct Request {
    pub path: Path,
    pub raw: HttpRequest,
}
