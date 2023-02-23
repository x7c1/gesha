mod generate;
mod test;
mod trace;

use clap::Parser;
use std::process::ExitCode;
use tracing::{error, info};
use Subcommand::{Generate, Test, TestOverwrite};

#[tokio::main]
async fn main() -> ExitCode {
    trace::init();

    let args: Args = Args::parse();
    info!("main> {:?}", args);

    let result = match args.sub {
        Generate(x) => generate::run(x),
        Test(x) => test::run(x).await,
        TestOverwrite(x) => test::overwrite::run(x),
    };
    let code = match result {
        Ok(_) => {
            info!("done");
            ExitCode::SUCCESS
        }
        Err(cause) => {
            let message = cause.dump();
            println!("[failed] {}", message);
            error!("{}", message);
            ExitCode::FAILURE
        }
    };
    trace::shutdown();
    code
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
    TestOverwrite(test::overwrite::Params),
}
