use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use crate::{assets::progress::Operation, cli::Cli};

use assets::progress::{BenchmarkProgressToml, ProgressError, FILE_NAME};
use blackheap_benchmarker::{AccessPattern, BenchmarkConfig, BenchmarkResults, ErrorCodes};
use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};
use tracing_subscriber::fmt::layer;

mod assets;
mod cli;

const ACCESS_SIZES: [u32; 24] = [
    2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072,
    262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216,
];

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
    results: HashMap<u32, Vec<f64>>,
}

impl Benchmark {
    pub fn get_all_benchmarks(root: bool, file_path: &str) -> Vec<Self> {
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

fn load_or_create_progress(
    directory_path: &Path,
    benchmarks: &[Benchmark],
) -> Result<BenchmarkProgressToml, ProgressError> {
    let mut full_path = PathBuf::from(directory_path);
    full_path.push(FILE_NAME);

    /* If it does not exist, create a new one based on our benchmarks */
    if !full_path.exists() {
        info!("No previous results were found. Creating new ones");
        let toml = BenchmarkProgressToml::new_from_benchmarks(benchmarks, &ACCESS_SIZES);
        toml.to_file(full_path.to_str().unwrap())?;
        return Ok(toml);
    }

    /* If it does exist, try to parse it */
    let toml = BenchmarkProgressToml::from_file(full_path.to_str().unwrap())?;
    info!("Previous results loaded");
    Ok(toml)
}

fn save_and_update_progress(
    b: &Benchmark,
    access_size: u32,
    results: &BenchmarkResults,
    cli: &Cli,
    progress: &mut BenchmarkProgressToml,
) -> Result<(), ProgressError> {
    let operation = Operation::from_is_read_op(b.config.is_read_operation).to_string();
    let file_path = format!(
        "{}/{}/{}/{}.txt",
        cli.to.to_str().unwrap(),
        b.scenario.to_string(),
        operation,
        access_size
    );

    /* we save it as newline seperated f64s */
    let durations_str = results
        .durations
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    /* save the file */
    fs::write(&file_path, &durations_str)?;

    /* Update the progress */
    progress.update_access_sizes_done(b, access_size);

    let progress_file_path = format!("{}/{}", cli.to.to_str().unwrap(), &FILE_NAME);
    progress.to_file(&progress_file_path)?;

    Ok(())
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
    info!("Trying to load previous results");
    let benchmarks = Benchmark::get_all_benchmarks(cli.drop_caches, cli.file.to_str().unwrap());
    let progress = load_or_create_progress(&cli.to, &benchmarks);
    if let Err(e) = progress {
        error!("{:?}", e);
        std::process::exit(1);
    }
    let mut progress = progress.unwrap();

    /* The actual benchmarking */
    for b in benchmarks.iter() {
        /* Which access sizes do we still have to do? */
        let missing_access_sizes = {
            let tmp_progress = progress.clone();
            tmp_progress
                .get_missing_access_sizes(&b)
                .map(|slice| slice.to_vec())
        };
        if None == missing_access_sizes {
            info!(
                "Benchmark {:?} ({:?}) already computed",
                &b.scenario,
                Operation::from_is_read_op(b.config.is_read_operation)
            );
            continue;
        }
        let missing_access_sizes: Vec<u32> = missing_access_sizes.unwrap();
        info!(
            "Benchmark {:?} ({:?}): Missing Access Sizes: {:?}",
            &b.scenario,
            Operation::from_is_read_op(b.config.is_read_operation),
            &missing_access_sizes
        );

        /* Do a benchmark for each access size */
        for access_size in missing_access_sizes {
            /* Set the access size */
            let mut config = b.config.clone();
            config.access_size_in_bytes = access_size as usize;

            /* Run the benchmark */
            info!(
                "Running {:?} ({:?}): Access Sizes: {:?}",
                &b.scenario,
                Operation::from_is_read_op(b.config.is_read_operation),
                access_size
            );
            let results = blackheap_benchmarker::benchmark_file(&config);
            if results.res != ErrorCodes::Success {
                info!(
                    "Error {:?} ({:?}): Access Sizes: {:?} failed with {:?}",
                    &b.scenario,
                    Operation::from_is_read_op(b.config.is_read_operation),
                    access_size,
                    &results.res
                );
            }

            /* Save the result; update and save the progress struct */
            info!("Saving the results");
            let res = save_and_update_progress(&b, access_size, &results, &cli, &mut progress);
            if let Err(e) = res {
                error!("{:?}", e);
                std::process::exit(1);
            }
        }
    }

    /* Do the regression for all benchmarks */

    /* Save the regression (should be part of impl Model) */

    /* Dump all assets for Analysis */

    /* Print out how to use the assets, refer to the README */
}
