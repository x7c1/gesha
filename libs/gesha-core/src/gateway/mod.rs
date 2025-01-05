mod reader;
pub use reader::file_to_string;

mod writer;
pub use writer::Writer;

mod diff;
pub use diff::detect_diff;

pub mod testing;
