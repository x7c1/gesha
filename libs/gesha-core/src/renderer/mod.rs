mod error;
pub use error::{Error, Result};

mod macros;
mod rust_type;

use std::io::Write;

pub trait Renderer {
    fn render<W: Write>(self, write: W) -> Result<()>;
}
