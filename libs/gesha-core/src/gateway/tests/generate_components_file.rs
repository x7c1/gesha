use crate::gateway::{TestCase, Writer};
use crate::renderer::Renderer;
use crate::{gateway, render};
use std::io::Write;

pub fn generate_module_file<A>(path: &str, cases: Vec<TestCase<A>>) -> gateway::Result<()> {
    let writer = Writer {
        path: path.into(),
        preamble: Some(
            "/*\n    Generated by 'gesha test' command; DO NOT EDIT BY HAND!\n*/".to_string(),
        ),
    };
    writer.create_file(ModuleFile { cases })?;
    Ok(())
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