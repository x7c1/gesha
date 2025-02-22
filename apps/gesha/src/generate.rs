use clap::Parser;
use gesha_core::conversions::Converter;
use gesha_core::io::{Reader, Writer};
use gesha_core::Result;
use gesha_rust_shapes::v3_0;
use tracing::instrument;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long)]
    schema: String,

    #[clap(long)]
    output: String,
}

#[instrument(name = "generate::run")]
pub async fn run(args: Args) -> Result<()> {
    let converter = v3_0::DocumentConverter::default();
    process(converter, args).await
}

async fn process<A: Converter>(converter: A, args: Args) -> Result<()> {
    let writer = Writer::new(&converter, args.output);
    let reader = Reader::new(&args.schema);
    let target = reader.open_target_type(&converter)?;
    writer.write_code(target)?;
    Ok(())
}
