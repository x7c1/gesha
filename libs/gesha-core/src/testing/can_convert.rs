pub trait CanConvert<From>: Sized {
    fn convert(x: From) -> Result<Self, ConversionError>;
}

#[derive(Debug)]
pub enum ConversionError {
    RustShape(gesha_rust_shapes::Error),
}
