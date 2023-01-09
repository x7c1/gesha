use gesha_core::gateway;
use gesha_core::gateway::Reader;
use gesha_core::targets::rust_type::Modules;
use openapi_types::v3_0;

#[derive(clap::Args, Debug)]
pub struct Params {
    #[clap(long)]
    schema: String,
}

pub fn run(params: Params) -> gateway::Result<()> {
    println!("generate> {:?}", params);

    let reader = Reader::new::<v3_0::Document>();
    let rust_types: Modules = reader.open_rust_type(params.schema)?;
    println!("components: {:#?}", rust_types);
    println!("(UNIMPLEMENTED)");
    Ok(())
}
