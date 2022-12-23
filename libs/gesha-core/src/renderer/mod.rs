mod error;

pub use error::{Error, Result};
use std::fs::File;

mod macros;
mod rust_type;

pub trait Renderer {
    fn render(self, write: File) -> Result<()>;
}
