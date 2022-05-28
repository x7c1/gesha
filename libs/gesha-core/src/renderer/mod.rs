mod error;
pub use error::{Error, Result};

mod macros;
pub(self) use macros::render;

mod rust_type;

use std::io::Write;

pub trait Renderer {
    fn render<W: Write>(self, write: W) -> Result<()>;
}
