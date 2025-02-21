mod error;
pub use crate::error::{DiffStyle, Error, ErrorTheme, Result};

pub mod conversions;
mod io;
pub mod trace;

#[cfg(feature = "testing")]
pub mod testing;
