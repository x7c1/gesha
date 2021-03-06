use crate::gateway::testing::{new_writer, TestCase};
use crate::renderer::Renderer;
use crate::{gateway, render};
use std::io::Write;
use std::path::PathBuf;

pub fn generate_module_file<A, P>(path: P, cases: Vec<TestCase<A>>) -> gateway::Result<()>
where
    P: Into<PathBuf>,
{
    let writer = new_writer(path);
    writer.create_file(ModuleFile { cases })
}

struct ModuleFile<A> {
    cases: Vec<TestCase<A>>,
}

impl<A> Renderer for ModuleFile<A> {
    fn render<W: Write>(self, mut write: W) -> crate::renderer::Result<()> {
        for case in self.cases.into_iter() {
            render! { write =>
                echo > "pub mod {name};", name = case.module_name;
            }
        }
        Ok(())
    }
}
