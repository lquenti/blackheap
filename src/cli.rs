use clap::{AppSettings, Parser, Subcommand};

// TODO: some have to be cwd, some path of io-benchmarker
// Probably we should use lazy_static
pub const DEFAULT_MODEL_PATH: &str = "./default-model";
pub const DEFAULT_BENCHMARK_FILE_PATH: &str = "/tmp/io_benchmark_test_file.dat";

pub const NAME: &str = "io-modeller";
pub const AUTHOR: &str = "Lars Quentin <lars.quentin@gwdg.de>";
pub const VERSION: &str = "0.1";
pub const ABOUT: &str = "A blackbox modeller for I/O-classification";

#[derive(Parser)]
#[clap(name = NAME, author = AUTHOR, version = VERSION, about = ABOUT, long_about = None)]
#[clap(global_setting(AppSettings::InferLongArgs))]
#[clap(global_setting(AppSettings::PropagateVersion))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new performance model
    CreateModel {
        /// Path to where the models will be saved.
        #[clap(short, long, default_value_t = String::from(DEFAULT_MODEL_PATH))]
        to: String,
        /// Path to where the benchmark should be done.
        #[clap(short, long, default_value_t = String::from(DEFAULT_BENCHMARK_FILE_PATH))]
        file: String,
        #[clap(short, long, required = true)]
        benchmarker: String,
    },
    /// Evaluate recorded I/O accesses according to previously created benchmark.
    UseModel {
        /// Path to model on which the performane will be evaluated on.
        #[clap(short, long, required = true)]
        model: String,
        /// Path to the recorded io accesses.
        #[clap(short, long, required = true)]
        file: String,
    },
}