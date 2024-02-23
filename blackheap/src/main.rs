use std::{collections::HashMap, io, path::{Path, PathBuf}};

use crate::cli::Cli;

use assets::progress::{BenchmarkProgressToml, ProgressError, FILE_NAME};
use blackheap_benchmarker::{AccessPattern, BenchmarkConfig};
use clap::Parser;
use serde::{Serialize, Deserialize};
use tracing::{debug, error, info};

mod assets;
mod cli;

const ACCESS_SIZES: [u32; 24] = [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216];

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
    results: HashMap<u32, Vec<f64>>
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
            results: HashMap::new(),
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
            results: HashMap::new(),
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
            results: HashMap::new(),
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
            results: HashMap::new(),
        }
    }
}

fn load_or_create_progress(directory_path: &Path, benchmarks: &[Benchmark]) -> Result<BenchmarkProgressToml, ProgressError> {
    let mut full_path = PathBuf::from(directory_path);
    full_path.push(FILE_NAME);

    /* If it does not exist, create a new one based on our benchmarks */
    if !full_path.exists() {
        let toml = BenchmarkProgressToml::new_from_benchmarks(benchmarks, &ACCESS_SIZES);
        toml.to_file(full_path.to_str().unwrap())?;
        return Ok(toml);
    }

    /* If it does exist, try to parse it */
    let toml = BenchmarkProgressToml::from_file(full_path.to_str().unwrap())?;
    Ok(toml)
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

    /* Load previous results */
    let benchmarks = Benchmark::get_all_benchmarks(cli.drop_caches, cli.file.to_str().unwrap());
    let progress = load_or_create_progress(&cli.to, &benchmarks);
    
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
    - run all benchmarks one by one
      - update afterwards
    - start analysis
      - TODO if the plotting libraries arent good enough dump a python script in there
        - lin reg should still be done in here
      - Maybe do that in general?
    */
}
