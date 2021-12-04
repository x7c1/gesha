use crate::errors::RequestError;
use actix_multipart::Field;
use actix_web::http::header::ContentDisposition;
use actix_web::web::Bytes;
use futures_util::TryStreamExt;

pub trait Content: Send {}

#[derive(Debug)]
pub struct StringContent(Vec<String>);
impl Content for StringContent {}

#[derive(Debug)]
pub struct BinaryContent(Vec<Bytes>);
impl Content for BinaryContent {}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FormDataField<A: Content> {
    content_disposition: ContentDisposition,
    content: A,
}

impl FormDataField<StringContent> {
    pub async fn from_string(
        mut field: Field,
        content_disposition: ContentDisposition,
    ) -> Result<Self, RequestError> {
        let mut chunks = vec![];
        while let Some(chunk) = field.try_next().await? {
            let content = String::from_utf8_lossy(&chunk);
            chunks.push(content.to_string());
        }
        let content = StringContent(chunks);
        Ok(FormDataField {
            content_disposition,
            content,
        })
    }
}

impl FormDataField<BinaryContent> {
    pub async fn from_binary(
        mut field: Field,
        content_disposition: ContentDisposition,
    ) -> Result<Self, RequestError> {
        let mut chunks = vec![];
        while let Some(chunk) = field.try_next().await? {
            chunks.push(chunk);
        }
        let content = BinaryContent(chunks);
        Ok(FormDataField {
            content_disposition,
            content,
        })
    }
}
