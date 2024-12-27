use crate::gateway;
use crate::gateway::testing::{new_writer, TestCase};
use gesha_rust_types::{ModuleDeclarations, ModuleName};
use std::fmt::Debug;
use std::path::PathBuf;

pub fn generate_module_file<A, P>(path: P, cases: Vec<TestCase<A>>) -> gateway::Result<()>
where
    A: Debug,
    P: Into<PathBuf> + Debug,
{
    let writer = new_writer(path);
    let decls = cases
        .into_iter()
        .map(|case| case.module_name)
        .map(ModuleName::new)
        .collect::<ModuleDeclarations>();

    writer.create_file(decls)
}
