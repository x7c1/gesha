mod generate;

use crate::generate::Args;
use clap::Parser;
use gesha_core::{gateway, trace};
use std::process::ExitCode;
use tracing::{error, info};

#[tokio::main]
async fn main() -> ExitCode {
    trace::init();

    let args = Args::parse();
    info!("gesha: {:?}", args);

    let result = generate::run(args);
    trace::shutdown();
    to_code(result)
}

fn to_code(result: gateway::Result<()>) -> ExitCode {
    match result {
        Ok(_) => {
            info!("gesha: done");
            ExitCode::SUCCESS
        }
        Err(cause) => {
            let message = cause.dump();
            error!("{message}");
            ExitCode::FAILURE
        }
    }
}
