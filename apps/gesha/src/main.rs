use clap::Parser;
use gesha_core::targets::rust::{Definition, ToRust};
use gesha_core::{open_document_file, open_v3_0_schemas_file, to_rust_modules};
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
    let document = open_document_file(args.schema).unwrap_or_else(|e| {
        println!("[failed] {:#?}", e);
        exit(1);
    });
    let rust_types = to_rust_modules(document);
    println!("components: {:#?}", rust_types);
}

fn generate_sample(args: GenerateSampleArgs) {
    println!("test> {:?}", args);
    let schemas = open_v3_0_schemas_file(args.schema).unwrap_or_else(|e| {
        println!("[failed] {:#?}", e);
        exit(1);
    });
    let rust_types: Vec<Definition> = ToRust::apply(schemas).unwrap_or_else(|e| {
        println!("[failed] {:#?}", e);
        exit(1);
    });
    println!("schemas: {:#?}", rust_types);
}
