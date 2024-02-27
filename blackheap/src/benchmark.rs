use crate::assets::progress::Operation;
use crate::assets::progress::{BenchmarkProgressToml, ProgressError, FILE_NAME};
use crate::cli::Cli;
use blackheap_benchmarker::{AccessPattern, BenchmarkConfig, BenchmarkResults};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use tracing::info;

const ACCESS_SIZES: [u32; 24] = [
    2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072,
    262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BenchmarkScenario {
    RandomUncached,
    SameOffset,
    Reverse,
}

impl ToString for BenchmarkScenario {
    fn to_string(&self) -> String {
        match self {
            BenchmarkScenario::SameOffset => "SameOffset".to_string(),
            BenchmarkScenario::RandomUncached => "RandomUncached".to_string(),
            BenchmarkScenario::Reverse => "Reverse".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Benchmark {
    pub scenario: BenchmarkScenario,
    pub config: BenchmarkConfig,
    pub results: HashMap<u32, Vec<f64>>,
}

impl Benchmark {
    pub fn get_all_benchmarks(root: bool, file_path: &str) -> Vec<Self> {
        vec![
            Self::new_random_uncached_read(file_path, root),
            Self::new_random_uncached_write(file_path, root),
            Self::new_reverse_read(file_path, root),
            Self::new_reverse_write(file_path, root),
            Self::new_same_offset_read(file_path),
            Self::new_same_offset_write(file_path),
        ]
    }

    pub fn new_reverse_read(file_path: &str, root: bool) -> Self {
        Benchmark {
            scenario: BenchmarkScenario::RandomUncached,
            config: BenchmarkConfig {
                filepath: file_path.to_string(),
                memory_buffer_in_bytes: 4 * 1024 * 1024 * 1024,
                file_size_in_bytes: 25 * 1024 * 1024 * 1024,
                access_size_in_bytes: 4 * 1024, /* any random value */
                number_of_io_op_tests: 1000,
                access_pattern_in_memory: AccessPattern::Reverse,
                access_pattern_in_file: AccessPattern::Reverse,
                is_read_operation: true,
                prepare_file_size: true,
                drop_cache_first: root,
                do_reread: false,
                restrict_free_ram_to: None,
            },
            results: HashMap::new(),
        }
    }

    pub fn new_reverse_write(file_path: &str, root: bool) -> Self {
        Benchmark {
            scenario: BenchmarkScenario::RandomUncached,
            config: {
                let mut config = Self::new_reverse_read(file_path, root).config;
                config.is_read_operation = false;
                config
            },
            results: HashMap::new(),
        }
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

pub fn load_or_create_progress(
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

pub fn save_and_update_progress(
    b: &Benchmark,
    access_size: u32,
    results: &BenchmarkResults,
    cli: &Cli,
    progress: &mut BenchmarkProgressToml,
) -> Result<(), ProgressError> {
    let operation = Operation::from_is_read_op(b.config.is_read_operation).to_string();
    let dir = format!(
        "{}/{}/{}",
        cli.to.to_str().unwrap(),
        b.scenario.to_string(),
        operation,
    );
    let file_path = format!("{}/{}.txt", &dir, access_size,);
    fs::create_dir_all(dir)?;

    /* If it already exists but we did still benchmark, it was most likely interrupted while writing... */
    if Path::new(&file_path).exists() {
        fs::remove_file(&file_path)?;
    }

    File::create(&file_path)?;

    /* we save it as newline seperated f64s */
    let durations_str = results
        .durations
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    /* save the file */
    fs::write(file_path, durations_str)?;

    /* Update the progress */
    progress.update_access_sizes_done(b, access_size);

    let progress_file_path = format!("{}/{}", cli.to.to_str().unwrap(), &FILE_NAME);
    progress.to_file(&progress_file_path)?;

    Ok(())
}
