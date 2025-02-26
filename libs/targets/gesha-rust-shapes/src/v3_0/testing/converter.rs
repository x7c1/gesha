use crate::v3_0::converter::{format_code, generate_components_code};
use gesha_core::conversions;
use gesha_core::conversions::{Output, Result};
use openapi_types::v3_0;
use std::path::Path;

#[derive(Clone, Debug, Default)]
pub struct ComponentsConverter {}

impl conversions::Converter for ComponentsConverter {
    type OpenApiType = v3_0::ComponentsObject;
    type TargetType = gesha_rust_types::SourceCode;

    fn convert(&self, src: Self::OpenApiType) -> Result<Output<Self::TargetType>> {
        generate_components_code(src)
    }

    fn format_code(&self, path: &Path) -> gesha_core::Result<String> {
        format_code(path)
    }
}
