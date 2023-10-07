use clap::Parser;
use gesha_core::gateway;
use gesha_core::gateway::Reader;
use gesha_rust_types::Modules;
use openapi_types::v3_0;
use tracing::log::info;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long)]
    schema: String,
}

pub fn run(args: Args) -> gateway::Result<()> {
    info!("generate> {:?}", args);

    let reader = Reader::new::<v3_0::Document>();
    let rust_types: Modules = reader.open_rust_type(args.schema)?;
    info!("components: {:#?}", rust_types);
    info!("(UNIMPLEMENTED)");
    Ok(())
}
