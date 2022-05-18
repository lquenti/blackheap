use clap::{ArgEnum,Parser, Subcommand};

pub const DEFAULT_MODEL_PATH: &str = "./default-model";
pub const DEFAULT_BENCHMARK_FILE_PATH: &str = "/tmp/blackheap_benchmark_test_file.dat";
pub const DEFAULT_REPORT_PATH: &str = "./report";

pub const NAME: &str = "blackheap";
pub const AUTHOR: &str = "Lars Quentin <lars.quentin@gwdg.de>";
pub const VERSION: &str = "0.1";
pub const ABOUT: &str = "A blackbox modeller for I/O-classification";

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum ModelEnum {
    Linear,
    ConstantLinear,
}

#[derive(Parser)]
#[clap(name = NAME, author = AUTHOR, version = VERSION, about = ABOUT, long_about = None)]
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
        /// Which PredictionModel to use
        #[clap(short, long, arg_enum, default_value_t = ModelEnum::ConstantLinear)]
        model: ModelEnum,
        /// Whether the benchmark can be skipped (already computed)
        #[clap(long)]
        analyze_only: bool,
        /// Path to the used benchmarker
        #[clap(short, long, required = true)]
        benchmarker: String,
        /// Whether root is required or not (used to drop caches)
        #[clap(short, long)]
        root: bool,
    },
    /// Evaluate recorded I/O accesses according to previously created benchmark.
    UseModel {
        /// Path to model on which the performane will be evaluated on.
        #[clap(short, long, required = true)]
        model: String,
        /// Path to the recorded io accesses.
        #[clap(short, long, required = true)]
        file: String,
        /// Output of report
        #[clap(short, long, default_value_t = String::from(DEFAULT_REPORT_PATH))]
        to: String,
    },
}
