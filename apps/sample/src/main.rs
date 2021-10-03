use sample_client::sample_server;
use sample_client_macros::DelegateSample;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    sample_server!(Api::new())
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[derive(DelegateSample)]
struct Api {
    foo: String,
}

impl Api {
    fn new() -> Self {
        Api {
            foo: "fooooo".to_string(),
        }
    }
}

impl Api {
    async fn index(&self, id: u32, name: String) -> String {
        println!("server internal field: {}", self.foo);
        format!("Hello {}! id:{}", name, id)
    }
}
