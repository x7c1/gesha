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

#[macro_export]
macro_rules! sample_server {
    ($x: expr) => {
        actix_web::HttpServer::new(|| {
            let delegator = sample_client::ApiDelegator::new($x);
            actix_web::App::new()
                .data(delegator)
                .service(sample_client::index)
        })
    };
}

#[macro_export]
macro_rules! sample_delegate {
    ($x: ident) => {
        #[async_trait::async_trait]
        impl sample_client::Api for $x {
            async fn index(&self, id: u32, name: String) -> String {
                $x::index(self, id, name).await
            }
        }
    };
}

#[get("/{id}/{name}/index.html")]
pub async fn index(
    api: web::Data<ApiDelegator>,
    web::Path((id, name)): web::Path<(u32, String)>,
) -> impl Responder {
    api.index(id, name).await
}
