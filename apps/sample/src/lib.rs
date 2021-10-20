use sample_server_derive::Sample;

mod index;
mod pet;

#[derive(Sample)]
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
