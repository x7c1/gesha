use clap::Parser;
use gesha_core::{open_document_file, translate_components};
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    schema: String,
}

fn main() {
    let args: Args = Args::parse();
    println!("main> {:?}!", args);

    let document = open_document_file(args.schema).unwrap_or_else(|e| {
        println!("[failed] {:#?}", e);
        exit(1);
    });

    let components = translate_components(document);
    println!("components: {:#?}", components);
}
