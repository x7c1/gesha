use crate::gateway;
use crate::gateway::{detect_diff, Reader, Writer};
use crate::targets::rust_type::Modules;
use openapi_types::v3_0;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TestTarget {
    pub schema: PathBuf,
    pub output: PathBuf,
    pub expected: PathBuf,
}

pub fn run_test(target: TestTarget) -> gateway::Result<()> {
    println!("target> {:#?}", target);

    let reader = Reader::new::<v3_0::ComponentsObject>();
    let rust_types: Modules = reader.open_rust_type(target.schema)?;
    println!("components: {:#?}", rust_types);

    let writer = Writer {
        path: target.output.clone(),
        preamble: None,
    };
    writer.print(rust_types)?;
    detect_diff(target.output, target.expected)?;
    Ok(())
}
