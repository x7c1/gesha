use handcraft_server_derive::Handcraft;

mod index;
mod pet;

#[derive(Handcraft)]
pub struct Handlers {
    foo: String,
}

impl Default for Handlers {
    fn default() -> Self {
        Handlers {
            foo: "fooooo".to_string(),
        }
    }
}
