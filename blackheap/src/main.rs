use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Model {
    Linear,
    ConstantLinear,
}

/// A blackbox modeller for I/O-classification
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Working directory for all the benchmarks and outputs.
    /// Also used to store progress.
    to: PathBuf,

    /// Path to where the benchmark should be done
    #[clap(short, long, default_value = "/tmp/blackheap_benchmark_test_file.dat")]
    file: PathBuf,

    /// Which PredictionModel to use
    #[clap(short, long, value_enum, default_value_t = Model::ConstantLinear)]
    model: Model,

    /// Drop caches (requires root)
    #[clap(long)]
    drop_caches: bool,

}

fn main() {
    human_panic::setup_panic!();
    let cli = Cli::parse();

    println!("{:?}", cli);

    /*
use blackheap_benchmarker::{BenchmarkConfig, AccessPattern};
    let cfg = BenchmarkConfig {
        filepath: String::from("/tmp/test_file.bin"),
        memory_buffer_in_bytes: 1024,
        file_size_in_bytes: 1024 * 10,
        access_size_in_bytes: 128,
        number_of_io_op_tests: 10,
        access_pattern_in_memory: AccessPattern::Sequential,
        access_pattern_in_file: AccessPattern::Sequential,
        is_read_operation: true,
        prepare_file_size: true,
        drop_cache_first: false,
        do_reread: false,
        restrict_free_ram_to: None
    };
    let res = blackheap_benchmarker::benchmark_file(&cfg);
    println!("{:?}", res.res);
    for val in res.durations {
        println!("{}", val);
    }
    */
}
