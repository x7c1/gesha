use clap::Parser;
use gesha_core::targets::rust::{Definition, Modules};
use gesha_core::Reader;
use openapi_types::v3_0;
use std::process::exit;

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
}

fn main() {
    let args: Args = Args::parse();
    println!("main> {:?}", args);

    match args.sub {
        Subcommand::Generate(x) => generate(x),
        Subcommand::GenerateSample(x) => generate_sample(x),
    }
}

fn generate(args: GenerateArgs) {
    println!("generate> {:?}", args);

    let reader = Reader::new::<v3_0::Document>();
    let rust_types: Option<Modules> = reader.open(args.schema).unwrap_or_else(|e| {
        println!("[failed] {:#?}", e);
        exit(1);
    });
    println!("components: {:#?}", rust_types);
}

fn generate_sample(args: GenerateSampleArgs) {
    println!("generate_sample> {:?}", args);

    let reader = Reader::new::<v3_0::SchemasObject>();
    let rust_types: Vec<Definition> = reader.open(args.schema).unwrap_or_else(|e| {
        println!("[failed] {:#?}", e);
        exit(1);
    });
    println!("schemas: {:#?}", rust_types);
}
