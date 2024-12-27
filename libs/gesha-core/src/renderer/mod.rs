mod error;
pub use error::{Error, Result};

use gesha_rust_types::Modules;
use std::io::Write;

pub trait Renderer {
    fn render(self, write: impl Write) -> Result<()>;
}

impl Renderer for Modules {
    fn render(self, mut write: impl Write) -> Result<()> {
        write!(write, "{}", &self)?;
        Ok(())
    }
}
