mod client;
use crate::client::{index, Api, ApiDelegator};

use actix_web::{App, HttpServer};
use async_trait::async_trait;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let delegator = ApiDelegator::new(ApiImpl {
            foo: "fooooo".to_string(),
        });
        App::new().data(delegator).service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

struct ApiImpl {
    foo: String,
}

#[async_trait]
impl Api for ApiImpl {
    async fn index(&self, id: u32, name: String) -> String {
        println!("server internal field: {}", self.foo);
        format!("Hello {}! id:{}", name, id)
    }
}
