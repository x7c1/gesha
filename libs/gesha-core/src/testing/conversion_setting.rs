use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ConversionSetting<From, To> {
    pub output: PathBuf,
    pub schema: PathBuf,
    pub example: PathBuf,
    pub(crate) phantom: PhantomData<(From, To)>,
}
