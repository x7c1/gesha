use sample_client::{petstore_server, petstore_delegate};

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

petstore_delegate!(ApiImpl);

impl ApiImpl {
    async fn index(&self, id: u32, name: String) -> String {
        println!("server internal field: {}", self.foo);
        format!("Hello {}! id:{}", name, id)
    }
}
