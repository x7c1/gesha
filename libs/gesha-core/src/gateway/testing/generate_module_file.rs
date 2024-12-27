use crate::gateway;
use crate::gateway::testing::{new_writer, TestCase};
use crate::renderer::Renderer;
use std::fmt::Debug;
use std::io::Write;
use std::path::PathBuf;

pub fn generate_module_file<A, P>(path: P, cases: Vec<TestCase<A>>) -> gateway::Result<()>
where
    A: Debug,
    P: Into<PathBuf> + Debug,
{
    let writer = new_writer(path);
    writer.create_file(ModuleFile { cases })
}

#[derive(Debug)]
struct ModuleFile<A> {
    cases: Vec<TestCase<A>>,
}

impl<A> Renderer for ModuleFile<A> {
    fn render(self, mut write: impl Write) -> crate::renderer::Result<()> {
        for case in self.cases.into_iter() {
            write!(write, "pub mod {name};", name = case.module_name)?;
        }
        Ok(())
    }
}
