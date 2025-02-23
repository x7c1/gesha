use crate::verify::Args;
use clap::Parser;
use gesha_core::{testing, trace, Result};
use std::process::ExitCode;
use tracing::{error, info};

mod overwrite;
mod verify;

#[tokio::main]
async fn main() -> ExitCode {
    testing::init();

    let args = Args::parse();
    info!("gesha-verify: {:#?}", args);

    let result = if args.overwrite {
        overwrite::run(args).await
    } else {
        verify::run(args).await
    };
    trace::wait_to_export().await;
    to_code(result)
}

fn to_code(result: Result<()>) -> ExitCode {
    match result {
        Ok(_) => {
            info!("gesha-verify: done");
            ExitCode::SUCCESS
        }
        Err(cause) => {
            let message = cause.dump();
            error!("{message}");
            ExitCode::FAILURE
        }
    }
}
