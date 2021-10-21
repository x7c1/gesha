use handcraft_server_derive::Handcraft;

mod index;
mod pet;

#[derive(Handcraft)]
pub struct Handlers {
    foo: String,
}

impl Handlers {
    pub fn new() -> Self {
        Handlers {
            foo: "fooooo".to_string(),
        }
    }
}
