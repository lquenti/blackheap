use crate::cli::Cli;

use blackheap_benchmarker::{AccessPattern, BenchmarkConfig};
use clap::Parser;
use serde::{Serialize, Deserialize};
use tracing::{debug, error, info};

mod assets;
mod cli;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BenchmarkScenario {
    RandomUncached,
    SameOffset,
}

impl ToString for BenchmarkScenario {
    fn to_string(&self) -> String {
        match self {
            BenchmarkScenario::SameOffset => "SameOffset".to_string(),
            BenchmarkScenario::RandomUncached => "RandomUncached".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct Benchmark {
    scenario: BenchmarkScenario,
    config: BenchmarkConfig,
}

impl Benchmark {
    pub fn get_all_benchmarks(
        root: bool,
        file_path: &str,
    ) -> Vec<Self> {
        vec![
            Self::new_random_uncached_read(file_path, root),
            Self::new_random_uncached_write(file_path, root),
            Self::new_same_offset_read(file_path),
            Self::new_same_offset_write(file_path),
        ]
    }

    pub fn new_random_uncached_read(file_path: &str, root: bool) -> Self {
        Benchmark {
            scenario: BenchmarkScenario::RandomUncached,
            config: BenchmarkConfig {
                filepath: file_path.to_string(),
                memory_buffer_in_bytes: 4 * 1024 * 1024 * 1024,
                file_size_in_bytes: 25 * 1024 * 1024 * 1024,
                access_size_in_bytes: 4 * 1024, /* any random value */
                number_of_io_op_tests: 1000,
                access_pattern_in_memory: AccessPattern::Random,
                access_pattern_in_file: AccessPattern::Random,
                is_read_operation: true,
                prepare_file_size: true,
                drop_cache_first: root,
                do_reread: false,
                restrict_free_ram_to: None,
            },
        }
    }

    pub fn new_random_uncached_write(file_path: &str, root: bool) -> Self {
        Benchmark {
            scenario: BenchmarkScenario::RandomUncached,
            config: {
                let mut config = Self::new_random_uncached_read(file_path, root).config;
                config.is_read_operation = false;
                config
            },
        }
    }

    pub fn new_same_offset_read(file_path: &str) -> Self {
        Benchmark {
            scenario: BenchmarkScenario::SameOffset,
            config: BenchmarkConfig {
                filepath: file_path.to_string(),
                memory_buffer_in_bytes: 4 * 1024 * 1024 * 1024,
                file_size_in_bytes: 25 * 1024 * 1024 * 1024,
                access_size_in_bytes: 4 * 1024, /* any random value */
                number_of_io_op_tests: 1000,
                access_pattern_in_memory: AccessPattern::Const,
                access_pattern_in_file: AccessPattern::Const,
                is_read_operation: true,
                prepare_file_size: true,
                drop_cache_first: false,
                do_reread: true,
                restrict_free_ram_to: None,
            },
        }
    }

    pub fn new_same_offset_write(file_path: &str) -> Self {
        Benchmark {
            scenario: BenchmarkScenario::SameOffset,
            config: {
                let mut config = Self::new_same_offset_read(file_path).config;
                config.is_read_operation = false;
                config
            },
        }
    }
}

fn main() {
    /* Init boilerplate */
    human_panic::setup_panic!();
    tracing_subscriber::fmt::init();

    /* CLI parsing */
    info!("Parsing and validating CLI");
    let cli = Cli::parse();
    debug!("{:?}", &cli);
    if let Err(e) = cli::validate_cli(&cli) {
        error!("{:?}", e);
        std::process::exit(1);
    }

    /* Create folder / Load old data */

    /*
    Old Logic:
    - Create output folder
    - dump static files
    - Create a vector of all performance benchmarks
    - For all benchmarks:
        - if not `analyze_only` run and save the benchmark
        - run the analysis
    - dump all to file
        - model.json
        - iofs.csv
    */

    /*
    New Logic:
    - try loading previous data => into Option<>
    - if not, create progress file (in toml)
    - run all benchmarks one by one
      - update afterwards
    - start analysis
      - TODO if the plotting libraries arent good enough dump a python script in there
        - lin reg should still be done in here
      - Maybe do that in general?
    */
}
