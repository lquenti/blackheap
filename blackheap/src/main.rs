use crate::cli::Cli;

use clap::Parser;

mod cli;


fn main() {
    human_panic::setup_panic!();
    let cli = Cli::parse();

    if let Err(e) = cli::validate_cli(&cli) {
        println!("{:?}", e);
    }

    println!("{:?}", cli);
}
