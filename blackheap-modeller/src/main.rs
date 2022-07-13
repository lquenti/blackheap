use clap::Parser;

mod analyzer;
mod benchmark_wrapper;
mod cli;
mod create_model;

use cli::Cli;

fn main() {
    let cli = Cli::parse();

    match create_model::create_model(
        &cli.to,
        &cli.file,
        &cli.benchmarker,
        cli.root,
        cli.analyze_only,
        cli.model,
    ) {
        Ok(_) => {}
        Err(e) => eprintln!("{:?}", e),
    }
}
