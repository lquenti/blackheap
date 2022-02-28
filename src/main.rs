// TODO: Replace unwraps and excepts
// TODO: Replace paths with AsRef<Path>
use clap::{IntoApp, Parser};



mod cli;
mod benchmark_wrapper;
mod analyzer;
mod subprograms;
mod html_templater;

use subprograms::use_model::use_model;
use subprograms::create_model::{validate_create_model, create_model};

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::CreateModel { to, file, benchmarker } => {
            if let Err(e) = validate_create_model(to, benchmarker) {
                let mut app = cli::Cli::into_app();
                app.error(
                    clap::ErrorKind::InvalidValue,
                    format!("{:?}", e)
                ).exit();
            }
            match create_model(to, file, benchmarker)  {
                Ok(_) => { },
                Err(e) => eprintln!("{:?}", e),
            }
        },
        cli::Commands::UseModel { model, file } => {
            match use_model(model, file) {
                Ok(_) => { },
                Err(e) => eprintln!("{:?}", e),
            }
        },
    }
}
