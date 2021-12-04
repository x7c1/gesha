use actix_multipart::Multipart;
use actix_web::body::Body;
use actix_web::{post, HttpRequest, HttpResponse};
use futures_util::TryStreamExt;

#[post("/multipart_request")]
pub async fn post_multipart_request(
    _raw: HttpRequest,
    mut payload: Multipart,
) -> actix_web::Result<HttpResponse> {
    println!("[start] post_multipart_request");

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field
            .content_disposition()
            .ok_or_else(|| HttpResponse::BadRequest().finish())?;

        let filename = content_disposition.get_filename().map_or_else(
            || "unknown-filename".to_string(),
            |f| sanitize_filename::sanitize(f),
        );
        let name = content_disposition
            .get_name()
            .unwrap_or_else(|| "unknown-name");

        println!("filename: {}", filename);
        println!("name: {}", name);

        while let Some(chunk) = field.try_next().await? {
            println!("len: {}", chunk.len());
            println!("chunk: {}", String::from_utf8_lossy(&chunk));
        }
    }

    let mut response = HttpResponse::Created();
    actix_web::Result::Ok(response.body(Body::Empty))
}
