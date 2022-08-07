// TODO
pub mod request_bodies {
    use super::core::Error;
    use super::core::MediaType;
    use super::core::Result;
    use super::schemas::Pet;
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PetBody {
        ApplicationXml(Pet),
        ApplicationJson(Pet),
    }

    impl PetBody {
        pub fn as_media_type(&self) -> &MediaType {
            match self {
                PetBody::ApplicationXml(_) => &MediaType::ApplicationXml,
                PetBody::ApplicationJson(_) => &MediaType::ApplicationJson,
            }
        }

        pub fn new(value: &str, media_type: &str) -> Result<Self> {
            match media_type {
                "application/xml" => todo!(),
                "application/json" => {
                    let body = serde_json::from_str(value).map_err(Error::InvalidJson)?;
                    Ok(Self::ApplicationJson(body))
                }
                unsupported => Err(Error::UnsupportedMediaType(unsupported.to_string())),
            }
        }
    }
}

pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Pet {
        pub id: i64,
    }
}

pub mod core {
    pub type Result<A> = std::result::Result<A, Error>;

    #[derive(Debug)]
    pub enum Error {
        InvalidJson(serde_json::Error),
        UnsupportedMediaType(String),
    }

    #[derive(Debug)]
    pub enum MediaType {
        ApplicationJson,
        ApplicationXml,
    }

    impl AsRef<str> for MediaType {
        fn as_ref(&self) -> &str {
            match self {
                MediaType::ApplicationJson => "application/json",
                MediaType::ApplicationXml => "application/xml",
            }
        }
    }
}
