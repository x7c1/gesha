use clap::Parser;
use gesha_core::io::{Reader, Writer};
use gesha_core::targets::rust_type::Modules;
use openapi_types::v3_0;
use std::process::exit;
use Subcommand::{Generate, GenerateSample};

fn main() {
    let args: Args = Args::parse();
    println!("main> {:?}", args);

    let result = match args.sub {
        Generate(x) => generate(x),
        GenerateSample(x) => generate_sample(x),
    };
    result.unwrap_or_else(|e| {
        println!("[failed] {:#?}", e);
        exit(1);
    });
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
    GenerateSample(GenerateSampleArgs),
}

#[derive(clap::Args, Debug)]
struct GenerateArgs {
    #[clap(long)]
    schema: String,
}

#[derive(clap::Args, Debug)]
struct GenerateSampleArgs {
    #[clap(long)]
    schema: String,

    #[clap(long)]
    output: String,
}

fn generate(args: GenerateArgs) -> gesha_core::Result<()> {
    println!("generate> {:?}", args);

    let reader = Reader::new::<v3_0::Document>();
    let rust_types: Modules = reader.open_rust_type(args.schema)?;
    println!("components: {:#?}", rust_types);
    Ok(())
}

fn generate_sample(args: GenerateSampleArgs) -> gesha_core::Result<()> {
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
