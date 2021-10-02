use actix_web::{get, web, Responder};
use async_trait::async_trait;

#[async_trait]
pub trait Api {
    async fn index(&self, id: u32, name: String) -> String;
}

pub struct ApiDelegator {
    api: Box<dyn Api>,
}

impl ApiDelegator {
    pub fn new<A: Api + 'static>(api: A) -> Self {
        ApiDelegator { api: Box::new(api) }
    }

    pub async fn index(&self, id: u32, name: String) -> String {
        self.api.index(id, name).await
    }
}

#[get("/{id}/{name}/index.html")]
pub async fn index(
    api: web::Data<ApiDelegator>,
    web::Path((id, name)): web::Path<(u32, String)>,
) -> impl Responder {
    api.index(id, name).await
}
