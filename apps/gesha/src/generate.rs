use gesha_core::gateway;
use gesha_core::gateway::Reader;
use openapi_types::v3_0;

#[derive(clap::Args, Debug)]
pub struct Params {
    #[clap(long)]
    schema: String,
}

pub fn run(params: Params) -> gateway::Result<()> {
    println!("generate> {:?}", params);

    let _reader = Reader::new::<v3_0::Document>();
    println!("(UNIMPLEMENTED)");
    Ok(())
}
