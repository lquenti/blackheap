use clap::Parser;

mod analyzer;
mod benchmark_wrapper;
mod cli;
mod frontend;
mod subprograms;

use cli::{Cli, Commands};
use subprograms::create_model;
use subprograms::use_model;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::CreateModel {
            to,
            file,
            benchmarker,
            root,
        } => match create_model::create_model(&to, &file, &benchmarker, root) {
            Ok(_) => {}
            Err(e) => eprintln!("{:?}", e),
        },
        Commands::UseModel { model, file, to } => match use_model::use_model(&model, &file, &to) {
            Ok(_) => {}
            Err(e) => eprintln!("{:?}", e),
        },
    }
}
