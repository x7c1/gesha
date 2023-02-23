mod test;
mod trace;

use crate::test::{overwrite, Args};
use clap::Parser;
use std::process::ExitCode;
use tracing::{error, info};

#[tokio::main]
async fn main() -> ExitCode {
    trace::init();

    let args = Args::parse();
    info!("main> {:?}", args);

    let result = if args.overwrite {
        // TODO: make it async
        overwrite::run(args)
    } else {
        test::run(args).await
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
