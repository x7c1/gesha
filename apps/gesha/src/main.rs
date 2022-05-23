use clap::Parser;
use gesha_core::io::Reader;
use gesha_core::renderer::Renderer;
use gesha_core::targets::rust_type::{Definition, Modules};
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
    let rust_types: Modules = reader.open_rust_type(args.schema).unwrap_or_else(|e| {
        println!("[failed] {:#?}", e);
        exit(1);
    });
    println!("components: {:#?}", rust_types);
}

fn generate_sample(args: GenerateSampleArgs) {
    println!("generate_sample> {:?}", args);

    let reader = Reader::new::<v3_0::SchemasObject>();
    let rust_types: Vec<Definition> = reader.open_rust_type(args.schema).unwrap_or_else(|e| {
        println!("[failed] {:#?}", e);
        exit(1);
    });
    println!("schemas: {:#?}", rust_types);

    let code = rust_types.render().unwrap_or_else(|e| {
        println!("[failed] cannot render: {:#?}", e);
        exit(1);
    });
    println!("rendered: {}", code)
}
