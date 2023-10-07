mod generate;
mod trace;

use crate::generate::Args;
use clap::Parser;
use std::process::ExitCode;
use tracing::{error, info};

#[tokio::main]
async fn main() -> ExitCode {
    trace::init();
    let args = Args::parse();
    info!("main> {:?}", args);

    let code = match generate::run(args) {
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
