// TODO
pub mod request_bodies {
    use serde::Deserialize;
    use serde::Serialize;

    /**
    Request body containing pet information
    */
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum PetBody {
        ApplicationJson(super::schemas::Pet),
    }

    impl PetBody {
        pub fn media_type(&self) -> super::core::MediaType {
            match self {
                Self::ApplicationJson(_) => super::core::MediaType::ApplicationJson,
            }
        }
        pub fn new(value: &str, media_type: &str) -> super::core::Result<Self> {
            match media_type {
                "application/json" => {
                    let body = super::core::from_json(value)?;
                    Ok(Self::ApplicationJson(body))
                }
                unsupported => Err(super::core::Error::UnsupportedMediaType {
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
            }
        }
    }

    pub fn from_json<'a, A: Deserialize<'a>>(text: &'a str) -> Result<A> {
        serde_json::from_str(text).map_err(Error::InvalidJson)
    }
}
