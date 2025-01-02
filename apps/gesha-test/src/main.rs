use crate::process::Args;
use clap::Parser;
use gesha_core::{trace, Result};
use std::process::ExitCode;
use tracing::{error, info};

mod overwrite;
mod process;

#[tokio::main]
async fn main() -> ExitCode {
    trace::init();

    let args = Args::parse();
    info!("gesha-test: {:?}", args);

    let result = if args.overwrite {
        overwrite::run(args).await
    } else {
        process::run(args).await
    };
    trace::shutdown();
    to_code(result)
}

fn to_code(result: Result<()>) -> ExitCode {
    match result {
        Ok(_) => {
            info!("gesha-test: done");
            ExitCode::SUCCESS
        }
        Err(cause) => {
            let message = cause.dump();
            error!("{message}");
            ExitCode::FAILURE
        }
    }
}
