use clap::Parser;
use omen::build_micro;

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opts {
    #[clap(short, long)]
    name: String,
}

fn main () -> anyhow::Result<()> {
    let opts : Opts = Opts::parse();
    build_micro(&opts.name)?;
    println!("Microservice '{}' generated succesfully", opts.name);

    Ok(())
}
