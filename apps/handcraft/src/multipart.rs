use actix_multipart::Multipart;
use actix_web::body::Body;
use actix_web::{post, HttpRequest, HttpResponse};
use actix_web::web::Buf;
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
        println!("filename: {}", filename);

        while let Some(chunk) = field.try_next().await? {
            println!("len: {}", chunk.len());
            println!("chunk: {}", String::from_utf8_lossy(&chunk));
            // filesystem operations are blocking, we have to use threadpool
            // f = web::block(move || f.write_all(&chunk).map(|_| f)).await?;
        }
    }
    let mut response = HttpResponse::Created();
    actix_web::Result::Ok(response.body(Body::Empty))
}
