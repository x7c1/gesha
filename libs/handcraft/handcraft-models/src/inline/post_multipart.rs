use crate::errors::RequestError;
use crate::schemas::{Error, MultipartFormDataParameters};
use actix_multipart::Multipart;
use actix_web::body::Body;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest, HttpResponse};
use futures_util::future::LocalBoxFuture;
use futures_util::FutureExt;

#[derive(Debug)]
pub struct Request {
    pub body: Result<MultipartFormDataParameters, RequestError>,
}

impl FromRequest for Request {
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Request, actix_web::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let multipart = Multipart::new(req.headers(), payload.take());
        MultipartFormDataParameters::from_multipart(multipart)
            .map(|body| Ok(Request { body }))
            .boxed_local()
    }
}

pub trait Responder {
    fn to_raw(self) -> HttpResponse;
}

#[derive(Debug)]
pub enum Response {
    Created,
    InternalServerError { content: Error },
}

impl Responder for Response {
    fn to_raw(self) -> HttpResponse {
        match self {
            Response::Created => {
                let mut response = HttpResponse::Created();
                response.body(Body::Empty)
            }
            Response::InternalServerError { content } => {
                let mut response = HttpResponse::InternalServerError();
                response.json(content)
            }
        }
    }
}
