use crate::process::Args;
use clap::Parser;
use gesha_core::trace;
use std::process::ExitCode;
use tracing::{error, info};

mod overwrite;
mod process;

#[tokio::main]
async fn main() -> ExitCode {
    trace::init();

    let args = Args::parse();
    info!("start: {:?}", args);

    let result = if args.overwrite {
        overwrite::run(args).await
    } else {
        process::run(args).await
    };
    let code = match result {
        Ok(_) => {
            info!("done");
            ExitCode::SUCCESS
        }
        Err(cause) => {
            let message = cause.dump();
            error!("{message}");
            ExitCode::FAILURE
        }
    };
    trace::shutdown();
    code
}
