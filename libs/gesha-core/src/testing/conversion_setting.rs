use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ConversionSetting<From, To> {
    pub output: PathBuf,
    pub schema: PathBuf,
    pub example: PathBuf,
    pub module_name: String,
    pub(crate) phantom: PhantomData<(From, To)>,
}
