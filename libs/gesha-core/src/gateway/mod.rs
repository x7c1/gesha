mod error;
pub use error::{Error, Result};

mod reader;
pub use reader::{file_to_string, Reader};

mod writer;
pub use writer::Writer;

mod diff;
pub use diff::detect_diff;
