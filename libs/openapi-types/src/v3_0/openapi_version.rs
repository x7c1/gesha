use crate::Output;
use crate::Unsupported::IncompatibleOpenApiVersion;

#[derive(Clone, Debug)]
pub struct OpenApiVersion(String);

impl OpenApiVersion {
    const DEFAULT: &'static str = "3.0.4";

    /// If the given version is not supported, the default version is used,
    /// and `IncompatibleVersion` is included as an error in `Output`.
    pub fn from_string(version: String) -> Output<OpenApiVersion> {
        if !version.starts_with("3.0.") {
            let error = IncompatibleOpenApiVersion { version };
            return Output::new(
                OpenApiVersion(Self::DEFAULT.to_string()),
                vec![error.into()],
            );
        }
        Output::ok(OpenApiVersion(version))
    }
}

impl AsRef<str> for OpenApiVersion {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<OpenApiVersion> for String {
    fn from(version: OpenApiVersion) -> Self {
        version.0
    }
}
