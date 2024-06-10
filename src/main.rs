
use clap::{Parser,Subcommand};
use log::info;

#[derive(Parser,Debug)]
struct Args {
    #[arg(long, help = "OAS File to load")]
    file: String,
}

fn main() {
    env_logger::init();

    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    info!("Starting {pkg} : v{ver}");
}
