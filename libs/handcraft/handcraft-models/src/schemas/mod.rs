use crate::core::{BinaryContent, FormDataField, ObjectContent, StringContent};
use crate::errors::RequestError;
use crate::errors::RequestError::{
    ContentDispositionNameNotFound, ContentDispositionNotFound, FormDataFieldRequired,
};
use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Pet {
    pub id: i64,
    pub name: String,
    pub tag: Option<String>,
}

impl From<Pet> for NewPet {
    fn from(pet: Pet) -> Self {
        NewPet {
            name: pet.name,
            tag: pet.tag,
        }
    }
}

impl NewPetLike for Pet {
    fn name(&self) -> &str {
        &self.name
    }

    fn tag(&self) -> &Option<String> {
        &self.tag
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Pets(pub Vec<Pet>);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NewPet {
    pub name: String,
    pub tag: Option<String>,
}

pub trait NewPetLike {
    fn name(&self) -> &str;
    fn tag(&self) -> &Option<String>;
}

impl NewPetLike for NewPet {
    fn name(&self) -> &str {
        &self.name
    }

    fn tag(&self) -> &Option<String> {
        &self.tag
    }
}

#[derive(Debug)]
pub struct MultipartFormDataParameters {
    pub string_field: FormDataField<StringContent>,
    pub binary_field: FormDataField<BinaryContent>,
    pub object_field: Option<FormDataField<ObjectContent<SampleObjectField>>>,
}

#[derive(Debug, Deserialize)]
pub struct SampleObjectField {
    pub field_a: String,
}

impl MultipartFormDataParameters {
    pub async fn from_multipart_form_data(mut multipart: Multipart) -> Result<Self, RequestError> {
        let mut string_field = None;
        let mut binary_field = None;
        let mut object_field = None;

        while let Some(field) = multipart.try_next().await? {
            let content_disposition = field
                .content_disposition()
                .ok_or(ContentDispositionNotFound)?;

            let name = content_disposition
                .get_name()
                .ok_or(ContentDispositionNameNotFound)?;

            match name {
                "string_field" => {
                    let field = FormDataField::from_string(field, content_disposition).await?;
                    string_field = Some(field)
                }
                "binary_field" => {
                    let field = FormDataField::from_binary(field, content_disposition).await?;
                    binary_field = Some(field)
                }
                "object_field" => {
                    let field = FormDataField::from_object(field, content_disposition).await?;
                    object_field = Some(field);
                }
                _ => (/* ignore unknown field */),
            };
        }
        Ok(MultipartFormDataParameters {
            string_field: string_field.ok_or_else(|| FormDataFieldRequired {
                name: "string_field".to_string(),
            })?,
            binary_field: binary_field.ok_or_else(|| FormDataFieldRequired {
                name: "binary_field".to_string(),
            })?,
            object_field,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MultiPartFormDataResponse {
    pub string_field: ReceivedString,
    pub binary_field: ReceivedBinary,
    pub optional_string_field: Option<ReceivedString>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ReceivedString {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ReceivedBinary {
    pub name: String,
    pub length: i64,
    pub file_name: Option<String>,
}
