mod error;

pub use error::{Error, Result};
use std::io::Write;

mod macros;
mod rust_type;

pub trait Renderer {
    fn render(self, write: impl Write) -> Result<()>;
}
