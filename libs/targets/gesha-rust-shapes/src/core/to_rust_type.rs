use gesha_rust_types::SourceCode;

pub trait ToRustType: Sized {
    fn apply(self) -> crate::Result<SourceCode>;
}
