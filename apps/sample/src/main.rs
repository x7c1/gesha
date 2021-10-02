use sample_client::{sample_server, sample_delegate};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    sample_server!(ApiImpl {
        foo: "fooooo".to_string(),
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

struct ApiImpl {
    foo: String,
}

sample_delegate!(ApiImpl);

impl ApiImpl {
    async fn index(&self, id: u32, name: String) -> String {
        println!("server internal field: {}", self.foo);
        format!("Hello {}! id:{}", name, id)
    }
}
