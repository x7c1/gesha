use gesha_core::gateway;
use gesha_core::gateway::{detect_diff, Reader, Writer};
use gesha_core::targets::rust_type::Modules;
use openapi_types::v3_0;
use std::path::PathBuf;

pub fn run_tests(args: RunTestsArgs) -> gateway::Result<()> {
    println!("generate_sample> {:?}", args);

    run(TestTarget {
        schema: "examples/v3.0/components/pet.yaml".into(),
        output: "output/v3.0/components/pet.rs".into(),
        expected: "examples/v3.0/components/pet.rs".into(),
    })
}

#[derive(clap::Args, Debug)]
pub struct RunTestsArgs {
    #[clap(long)]
    schema: String,

    #[clap(long)]
    output: String,
}

#[derive(Debug)]
pub struct TestTarget {
    schema: PathBuf,
    output: PathBuf,
    expected: PathBuf,
}

fn run(target: TestTarget) -> gateway::Result<()> {
    println!("target> {:?}", target);

    let reader = Reader::new::<v3_0::ComponentsObject>();
    let rust_types: Modules = reader.open_rust_type(target.schema)?;

    println!("components: {:#?}", rust_types);

    let writer = Writer {
        path: target.output.clone(),
        preamble: None,
    };
    writer.print(rust_types)?;

    println!("expected: {:?}", target.expected);

    detect_diff(target.output, target.expected)?;

    Ok(())
}
