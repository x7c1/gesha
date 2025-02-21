use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TestCase<A> {
    pub output: PathBuf,
    pub schema: PathBuf,
    pub example: PathBuf,
    pub module_name: String,
    pub phantom: PhantomData<A>,
}

impl<A> Clone for TestCase<A> {
    fn clone(&self) -> Self {
        Self {
            output: self.output.clone(),
            schema: self.schema.clone(),
            example: self.example.clone(),
            module_name: self.module_name.clone(),
            phantom: PhantomData,
        }
    }
}
