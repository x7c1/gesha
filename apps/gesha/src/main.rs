mod generate;
mod test;

use clap::Parser;
use std::process::exit;
use Subcommand::{Generate, Test};

fn main() {
    let args: Args = Args::parse();
    println!("main> {:?}", args);

    let result = match args.sub {
        Generate(x) => generate::run(x),
        Test(x) => test::run(x),
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
    Generate(generate::Params),
    Test(test::Params),
}
