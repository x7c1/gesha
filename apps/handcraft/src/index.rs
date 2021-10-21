use crate::Handlers;
use handcraft_models::inline::index;
use handcraft_server_derive::assert_signature;

impl Handlers {
    #[assert_signature]
    pub async fn index(&self, req: index::Request) -> String {
        println!("server internal field: {}", self.foo);
        println!("request: {:#?}", req);
        format!("Hello {}! id:{}", req.path.name, req.path.id)
    }
}
