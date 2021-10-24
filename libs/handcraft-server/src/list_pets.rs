use actix_web::{HttpRequest, HttpResponse};
use handcraft_models::inline::{list_pets, RequestError};
use std::future::Future;

pub async fn delegate<F, G, X1, X2>(
    raw: HttpRequest,
    operate: F,
    on_bad_request: G,
) -> actix_web::Result<HttpResponse>
where
    F: Fn(list_pets::Request) -> X1,
    X1: Future<Output = X2>,
    X2: list_pets::Responder,
    G: Fn(RequestError) -> HttpResponse,
{
    let raw_response = match list_pets::Request::from_raw(raw).await {
        Ok(request) => {
            let response = operate(request).await;
            list_pets::Responder::to_raw(response)
        }
        Err(e) => on_bad_request(e),
    };
    actix_web::Result::Ok(raw_response)
}
