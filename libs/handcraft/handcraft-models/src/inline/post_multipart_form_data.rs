use crate::errors::RequestError;
use crate::schemas::{Error, MultiPartFormDataResponse, MultipartFormDataParameters};
use actix_multipart::Multipart;
use actix_web::body::BoxBody;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest, HttpResponse, Responder};
use futures_util::future::LocalBoxFuture;
use futures_util::FutureExt;

#[derive(Debug)]
pub struct Request {
    pub body: Result<MultipartFormDataParameters, RequestError>,
    pub raw: HttpRequest,
}

impl FromRequest for Request {
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Request, actix_web::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let multipart = Multipart::new(req.headers(), payload.take());
        let raw = req.clone();
        MultipartFormDataParameters::from_multipart_form_data(multipart)
            .map(|body| Ok(Request { body, raw }))
            .boxed_local()
    }
}

#[derive(Debug)]
pub enum Response {
    Created { content: MultiPartFormDataResponse },
    InternalServerError { content: Error },
}

impl Responder for Response {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        match self {
            Response::Created { content } => {
                let mut response = HttpResponse::Created();
                response.json(content)
            }
            Response::InternalServerError { content } => {
                let mut response = HttpResponse::InternalServerError();
                response.json(content)
            }
        }
    }
}
