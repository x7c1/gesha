use actix_web::{HttpRequest, HttpResponse};
use handcraft_models::inline::{show_pet_by_id, RequestError};
use std::future::Future;

pub async fn delegate<F, G, X1, X2>(
    raw: HttpRequest,
    operate: F,
    on_bad_request: G,
) -> actix_web::Result<HttpResponse>
where
    F: Fn(show_pet_by_id::Request) -> X1,
    X1: Future<Output = X2>,
    X2: show_pet_by_id::Responder,
    G: Fn(RequestError) -> HttpResponse,
{
    let raw_response = match show_pet_by_id::Request::from_raw(raw).await {
        Ok(request) => {
            let response = operate(request).await;
            show_pet_by_id::Responder::to_raw(response)
        }
        Err(e) => on_bad_request(e),
    };
    actix_web::Result::Ok(raw_response)
}
