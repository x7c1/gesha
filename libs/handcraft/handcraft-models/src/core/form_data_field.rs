use crate::errors::RequestError;
use crate::errors::RequestError::{ContentDispositionNameNotFound, JsonFormatError};
use actix_multipart::Field;
use actix_web::http::header::ContentDisposition;
use actix_web::web::Bytes;
use futures_util::TryStreamExt;
use serde::de::DeserializeOwned;
use std::fmt::{Debug, Display, Formatter};

#[allow(dead_code)]
pub trait Content: Send {}

#[derive(Debug)]
pub struct StringContent(Vec<String>);
impl Content for StringContent {}

#[derive(Debug)]
pub struct BinaryContent(Vec<Bytes>);
impl Content for BinaryContent {}

#[derive(Debug)]
pub struct ObjectContent<A: Send>(A);
impl<A: Send> Content for ObjectContent<A> {}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FormDataField<A> {
    content_disposition: ContentDisposition,
    content: A,
}

impl<A> FormDataField<A> {
    pub fn name(&self) -> Result<&str, RequestError> {
        self.content_disposition
            .get_name()
            .ok_or(ContentDispositionNameNotFound)
    }
}

impl<A: Send + Debug + DeserializeOwned> FormDataField<ObjectContent<A>> {
    pub async fn from_object(
        field: Field,
        content_disposition: ContentDisposition,
    ) -> Result<Self, RequestError> {
        let field = FormDataField::from_string(field, content_disposition).await?;
        let name = field.name()?;
        let object = serde_json::from_str(&field.to_string()).map_err(|e| JsonFormatError {
            key: name.to_string(),
            message: e.to_string(),
        })?;
        Ok(FormDataField {
            content_disposition: field.content_disposition,
            content: ObjectContent(object),
        })
    }
    pub fn extract(self) -> A {
        self.content.0
    }
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

impl Display for FormDataField<StringContent> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content.0.join(""))
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

    pub fn len(&self) -> usize {
        let mut len = 0;
        for x in &self.content.0 {
            len += x.len()
        }
        len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn file_name(&self) -> Option<&str> {
        self.content_disposition.get_filename()
    }
}
