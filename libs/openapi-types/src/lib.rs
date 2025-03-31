pub mod json_schema;
pub mod v3_0;
pub mod v3_x;
pub mod yaml;

mod error;
pub use error::{Error, Output, Result, SpecViolation, Unsupported};

pub(crate) use error::by_key;
