use clap::Parser;
use gesha_core::Result;
use tracing::log::info;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long)]
    schema: String,
}

pub fn run(args: Args) -> Result<()> {
    info!("generate> {:?}", args);
    info!("(UNIMPLEMENTED)");
    Ok(())
}
