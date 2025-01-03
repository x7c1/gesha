pub trait CanConvert<From>: Sized {
    fn convert(x: From) -> crate::Result<Self>;
}
