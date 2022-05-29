mod tests;

use clap::Parser;
use gesha_core::gateway;
use gesha_core::gateway::Reader;
use gesha_core::targets::rust_type::Modules;
use openapi_types::v3_0;
use std::process::exit;
use Subcommand::{Generate, Test};

fn main() {
    let args: Args = Args::parse();
    println!("main> {:?}", args);

    let result = match args.sub {
        Generate(x) => generate(x),
        Test => tests::run_tests(),
    };
    result.unwrap_or_else(|cause| {
        cause.dump();
        exit(1);
    });
    println!("[done]");
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    sub: Subcommand,
}

#[derive(clap::Subcommand, Debug)]
enum Subcommand {
    Generate(GenerateArgs),
    Test,
}

#[derive(clap::Args, Debug)]
struct GenerateArgs {
    #[clap(long)]
    schema: String,
}

fn generate(args: GenerateArgs) -> gateway::Result<()> {
    println!("generate> {:?}", args);

    let reader = Reader::new::<v3_0::Document>();
    let rust_types: Modules = reader.open_rust_type(args.schema)?;
    println!("components: {:#?}", rust_types);
    Ok(())
}
