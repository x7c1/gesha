mod generate;
use generate::{run_generate, GenerateArgs};

mod test;
use test::run_tests;

use clap::Parser;
use std::process::exit;
use Subcommand::{Generate, Test};

fn main() {
    let args: Args = Args::parse();
    println!("main> {:?}", args);

    let result = match args.sub {
        Generate(x) => run_generate(x),
        Test => run_tests(),
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
