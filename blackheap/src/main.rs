use crate::cli::Cli;

use tracing::info;

use clap::Parser;

mod cli;


fn main() {
    human_panic::setup_panic!();
    tracing_subscriber::fmt::init();

    info!("Parsing and validating CLI");
    let cli = Cli::parse();
    if let Err(e) = cli::validate_cli(&cli) {
        println!("{:?}", e);
    }

    println!("{:?}", cli);
}
