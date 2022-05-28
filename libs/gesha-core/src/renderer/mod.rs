mod error;
pub use error::{Error, Result};

mod rust_type;

pub trait Renderer {
    fn render(self) -> Result<String>;
}
