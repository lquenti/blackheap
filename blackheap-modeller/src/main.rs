use clap::Parser;

mod analyzer;
mod assets;
mod benchmark_wrapper;
mod cli;
mod create_model;

use cli::Cli;

fn main() {
    let cli: Cli = Cli::parse();

    if let Err(e) =
        create_model::create_model(&cli.to, &cli.file, cli.root, cli.analyze_only, cli.model)
    {
        eprintln!("{:?}", e)
    }
}
