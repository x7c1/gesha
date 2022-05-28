use gesha_core::gateway;
use gesha_core::gateway::{Reader, Writer};
use gesha_core::targets::rust_type::Modules;
use openapi_types::v3_0;

pub fn run_tests(args: RunTestsArgs) -> gateway::Result<()> {
    println!("generate_sample> {:?}", args);

    let reader = Reader::new::<v3_0::ComponentsObject>();
    let rust_types: Modules = reader.open_rust_type(args.schema)?;

    println!("components: {:#?}", rust_types);

    let writer = Writer {
        path: args.output.into(),
        preamble: None,
    };
    writer.print(rust_types)?;

    println!("[done]");
    Ok(())
}

#[derive(clap::Args, Debug)]
pub struct RunTestsArgs {
    #[clap(long)]
    schema: String,

    #[clap(long)]
    output: String,
}
