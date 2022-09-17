// TODO
pub mod request_bodies {
    use super::core::from_json_string;
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
        pub fn media_type(&self) -> MediaType {
            match self {
                Self::ApplicationXml(_) => MediaType::ApplicationXml,
                Self::ApplicationJson(_) => MediaType::ApplicationJson,
            }
        }

        pub fn new(value: &str, media_type: &str) -> Result<Self> {
            match media_type {
                "application/xml" => unimplemented!(),
                "application/json" => {
                    let body = from_json_string(value)?;
                    Ok(Self::ApplicationJson(body))
                }
                unsupported => Err(Error::UnsupportedMediaType {
                    given: unsupported.to_string(),
                }),
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
    use serde::Deserialize;
    use std::fmt::{Display, Formatter};

    pub type Result<A> = std::result::Result<A, Error>;

    #[derive(Debug)]
    pub enum Error {
        InvalidJson(serde_json::Error),
        UnsupportedMediaType { given: String },
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum MediaType {
        ApplicationJson,
        ApplicationXml,
    }

    impl Display for MediaType {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            Display::fmt(self.as_ref(), f)
        }
    }

    impl AsRef<str> for MediaType {
        fn as_ref(&self) -> &str {
            match self {
                MediaType::ApplicationJson => "application/json",
                MediaType::ApplicationXml => "application/xml",
            }
        }
    }

    pub fn from_json_string<'a, A: Deserialize<'a>>(text: &'a str) -> Result<A> {
        serde_json::from_str(text).map_err(Error::InvalidJson)
    }
}
