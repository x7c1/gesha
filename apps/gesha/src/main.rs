use clap::Parser;
use gesha_core::io::{write, Reader};
use gesha_core::targets::rust_type;
use gesha_core::targets::rust_type::Modules;
use openapi_types::v3_0;
use std::process::exit;
use Subcommand::{Generate, GenerateSample};

fn main() {
    let args: Args = Args::parse();
    println!("main> {:?}", args);

    match args.sub {
        Generate(x) => generate(x),
        GenerateSample(x) => generate_sample(x),
    }
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

fn generate(args: GenerateArgs) {
    println!("generate> {:?}", args);

    let rust_types: rust_type::Modules = Reader::new::<v3_0::Document>()
        .open_rust_type(args.schema)
        .unwrap_or_else(|e| {
            println!("[failed] {:#?}", e);
            exit(1);
        });

    println!("components: {:#?}", rust_types);
}

fn generate_sample(args: GenerateSampleArgs) {
    println!("generate_sample> {:?}", args);

    let rust_types: Modules = Reader::new::<v3_0::ComponentsObject>()
        .open_rust_type(args.schema)
        .unwrap_or_else(|e| {
            println!("[failed] {:#?}", e);
            exit(1);
        });

    println!("schemas: {:#?}", rust_types);

    write(args.output, rust_types).unwrap_or_else(|e| {
        println!("[failed] cannot write: {:#?}", e);
        exit(1);
    });
    println!("[done]")
}
