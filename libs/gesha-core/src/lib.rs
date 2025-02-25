mod error;
pub use crate::error::{DiffStyle, Error, ErrorTheme, Output, Result};

pub mod conversions;
pub mod io;
pub mod trace;

#[cfg(feature = "testing")]
pub mod testing;
