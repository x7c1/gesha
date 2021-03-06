use crate::Handlers;
use handcraft_models::inline::{add_pet, create_pets, find_pets, list_pets, show_pet_by_id};
use handcraft_models::schemas::{Error, Pet, Pets};
use handcraft_server_derive::assert_signature;

impl Handlers {
    #[assert_signature]
    pub async fn show_pet_by_id(
        &self,
        req: show_pet_by_id::Request,
    ) -> impl show_pet_by_id::Responder {
        println!("request: {:#?}", req);
        match req.path.pet_id.parse() {
            Ok(id) => show_pet_by_id::Response::OK {
                content: Pet {
                    id,
                    name: "handcraft_name".to_string(),
                    tag: None,
                },
            },
            Err(e) => show_pet_by_id::Response::InternalServerError {
                content: Error {
                    code: 1,
                    message: e.to_string(),
                },
            },
        }
    }
    #[assert_signature]
    pub async fn list_pets(&self, req: list_pets::Request) -> impl list_pets::Responder {
        println!("request: {:#?}", req);
        match req.query.limit {
            Some(123) => list_pets::Response::OK {
                headers: list_pets::ResponseHeaders {
                    x_next: Some("456".to_string()),
                },
                content: Pets(vec![
                    Pet {
                        id: 111,
                        name: "name-111".to_string(),
                        tag: None,
                    },
                    Pet {
                        id: 222,
                        name: "name-222".to_string(),
                        tag: None,
                    },
                ]),
            },
            Some(666) => list_pets::Response::InternalServerError {
                content: Error {
                    code: 333,
                    message: "sample error message".to_string(),
                },
            },
            _ => list_pets::Response::OK {
                headers: list_pets::ResponseHeaders { x_next: None },
                content: Pets(vec![]),
            },
        }
    }
    #[assert_signature]
    pub async fn create_pets(&self, req: create_pets::Request) -> impl create_pets::Responder {
        println!("request: {:#?}", req);
        create_pets::Response::Created
    }
    #[assert_signature]
    pub async fn find_pets(&self, req: find_pets::Request) -> impl find_pets::Responder {
        let pets = req
            .query
            .tags
            .iter()
            .enumerate()
            .map(|(i, tag)| Pet {
                id: i as i64,
                name: format!("name-{}", i),
                tag: Some(tag.to_string()),
            })
            .collect();

        find_pets::Response::OK {
            content: Pets(pets),
        }
    }
    #[assert_signature]
    pub async fn add_pet(&self, req: add_pet::Request) -> impl add_pet::Responder {
        add_pet::Response::OK {
            content: Pet {
                id: 123,
                name: format!("created:{}", req.body.name),
                tag: req.body.tag.map(|tag| format!("created:{}", tag)),
            },
        }
    }
}

/*
use actix_web::{get, web, HttpRequest, HttpResponse, FromRequest, Responder};
use handcraft_models::inline;

#[get("/pets/{pet_id}")]
pub async fn draft_show_pet_by_id(
    handlers: web::Data<Handlers>,
    raw: HttpRequest,
    path: web::Path<inline::show_pet_by_id::Path>,
) -> actix_web::Result<HttpResponse> {
    let request = inline::show_pet_by_id::Request {
        path: path.into_inner(),
        raw,
    };
    let response = handlers.show_pet_by_id(request).await;
    let raw_response = inline::show_pet_by_id::Responder::to_raw(response);
    actix_web::Result::Ok(raw_response)
}
// */
