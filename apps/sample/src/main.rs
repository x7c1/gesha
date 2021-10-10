use sample_server::sample_server;
use sample_server_derive::assert_signature;
use sample_server_derive::Sample;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    sample_server!(Api::new())
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[derive(Sample)]
pub struct Api {
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
    #[assert_signature(index)]
    async fn index(&self, id: u32, name: String) -> String {
        println!("server internal field: {}", self.foo);
        format!("Hello {}! id:{}", name, id)
    }
}
