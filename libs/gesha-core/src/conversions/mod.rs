mod error;
pub use error::{Error, Result};

mod reify;
use reify::{reify_entry, reify_value};

mod v3_0;

use crate::yaml_wrapper::YamlMap;

/// convert YamlMap to a type defined in openapi_types crate.
pub trait ToOpenApi: Sized {
    fn apply(map: YamlMap) -> Result<Self>;
}

/// convert A to a type defined in crate::targets::rust_type module.
pub trait ToRustType<A>: Sized {
    fn apply(this: A) -> Result<Self>;
}
