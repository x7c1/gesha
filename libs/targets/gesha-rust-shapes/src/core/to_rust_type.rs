/// convert A to a type defined in gesha_rust_types.
pub trait ToRustType<A>: Sized {
    fn apply(this: A) -> crate::Result<Self>;
}
