use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    schema: String,
}

fn main() {
    let args: Args = Args::parse();

    println!("schema: {}!", args.schema)
}
