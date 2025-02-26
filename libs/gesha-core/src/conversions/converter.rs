use crate::conversions::{Output, Result};
use openapi_types::yaml::ToOpenApi;
use std::fmt::{Debug, Display};
use std::path::Path;

/// Convert OpenAPI definitions to target type.
pub trait Converter: Debug + Clone + Sized + Send + Sync + 'static {
    /// The OpenAPI type that this definition converts from.
    type OpenApiType: ToOpenApi + Debug + Send + Sync;

    /// The target type that this definition converts to.
    type TargetType: Display + Send + Sync;

    /// Convert the given OpenAPI type to the target type.
    fn convert(&self, src: Self::OpenApiType) -> Result<Output<Self::TargetType>>;

    /// Format the code in the given path.
    fn format_code(&self, path: &Path) -> crate::Result<String>;
}
