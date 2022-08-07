// TODO
pub mod request_bodies {
    use super::core::MediaType;
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
