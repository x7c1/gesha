mod error;
pub use error::{Error, Result};

mod v3_0;

/// convert A to a type defined in gesha_rust_types module.
pub trait ToRustType<A>: Sized {
    fn apply(this: A) -> Result<Self>;
}
