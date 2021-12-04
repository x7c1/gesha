use actix_web::body::Body;
use actix_web::{post, HttpRequest, HttpResponse};
use handcraft_models::inline::post_multipart;

#[post("/multipart_form_data")]
pub async fn post_multipart_form_data(
    _raw: HttpRequest,
    request: post_multipart::Request,
) -> actix_web::Result<HttpResponse> {
    println!("[start] post_multipart_form_data: {:#?}", request);
    let mut response = HttpResponse::Created();
    println!("res: {:#?}", response);
    actix_web::Result::Ok(response.body(Body::Empty))
}
