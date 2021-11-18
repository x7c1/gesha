use actix_web::body::Body;
use actix_web::{post, HttpRequest, HttpResponse};

#[post("/multipart_request")]
pub async fn post_multipart_request(_raw: HttpRequest) -> actix_web::Result<HttpResponse> {
    let mut response = HttpResponse::Created();
    actix_web::Result::Ok(response.body(Body::Empty))
}
