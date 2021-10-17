use crate::Handlers;
use sample_models::inline::index;
use sample_server_derive::define;

impl Handlers {
    #[define]
    pub async fn index(&self, req: index::Request) -> String {
        println!("server internal field: {}", self.foo);
        println!("request: {:#?}", req);
        format!("Hello {}! id:{}", req.path.name, req.path.id)
    }
}
