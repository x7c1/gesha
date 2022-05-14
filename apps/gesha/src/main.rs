use clap::Parser;
use gesha_core::open_document_file;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    schema: String,
}

fn main() {
    let args: Args = Args::parse();
    println!("main> {:?}!", args);

    let document = open_document_file(args.schema);
    println!("document: {:#?}", document);
}
