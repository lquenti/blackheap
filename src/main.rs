use clap::Parser;

mod cli;
mod benchmark_wrapper;
mod analyzer;
mod subprograms;
mod html_templater;

use cli::{Cli, Commands};
use subprograms::use_model;
use subprograms::create_model;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::CreateModel { to, file, benchmarker, root } => {
            match create_model::create_model(to, file, benchmarker, root)  {
                Ok(_) => { },
                Err(e) => eprintln!("{:?}", e),
            }
        },
        Commands::UseModel { model, file } => {
            match use_model::use_model(model, file) {
                Ok(_) => { },
                Err(e) => eprintln!("{:?}", e),
            }
        },
    }
}
