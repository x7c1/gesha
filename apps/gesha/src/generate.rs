use clap::Parser;
use gesha_core::conversions::Generator;
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
    let generator = Generator::new(&converter, args.output);
    generator.generate_from_file(args.schema)
}
