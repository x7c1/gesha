mod petstore_client;
use crate::petstore_client::Api;

use async_trait::async_trait;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    petstore_server!(ApiImpl {
        foo: "fooooo".to_string(),
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
