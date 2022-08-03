use clap::{ArgEnum, Parser};

pub const DEFAULT_MODEL_PATH: &str = "./default-model";
pub const DEFAULT_BENCHMARK_FILE_PATH: &str = "/tmp/blackheap_benchmark_test_file.dat";

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
    /// Path to where the models will be saved.
    #[clap(short, long, default_value_t = String::from(DEFAULT_MODEL_PATH))]
    pub to: String,
    /// Path to where the benchmark should be done.
    #[clap(short, long, default_value_t = String::from(DEFAULT_BENCHMARK_FILE_PATH))]
    pub file: String,
    /// Which PredictionModel to use
    #[clap(short, long, arg_enum, default_value_t = ModelEnum::ConstantLinear)]
    pub model: ModelEnum,
    /// Whether the benchmark can be skipped (already computed)
    #[clap(long)]
    pub analyze_only: bool,
    /// Whether root is required or not (used to drop caches)
    #[clap(short, long)]
    pub root: bool,
}
