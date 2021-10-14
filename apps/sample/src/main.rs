use sample_server::{sample_server, IndexRequest};
use sample_server_derive::define;
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
    #[define(index)]
    async fn index(&self, req: IndexRequest) -> String {
        println!("server internal field: {}", self.foo);
        println!("request: {:#?}", req);
        format!("Hello {}! id:{}", req.path.name, req.path.id)
    }
}
