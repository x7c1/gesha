use std::marker::PhantomData;
use std::path::PathBuf;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct TestCase<From, To> {
    pub output: PathBuf,
    pub schema: PathBuf,
    pub example: PathBuf,
    pub module_name: String,
    pub phantom: PhantomData<(From, To)>,
}
