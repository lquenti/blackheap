use std::fs::{self, DirEntry, File, ReadDir};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::benchmark_wrapper::PerformanceBenchmark;

use anyhow::Result;
use serde::{Deserialize, Serialize};

fn get_all_jsons_from_directory(folder: &Path) -> Result<Vec<PathBuf>> {
    let folder: PathBuf = fs::canonicalize(&folder)?;
    let dir: ReadDir = fs::read_dir(&folder)?;

    let mut valid_dir_entries: Vec<DirEntry> = Vec::new();
    for dir_entry in dir {
        match dir_entry {
            Ok(d) => {
                valid_dir_entries.push(d);
            }
            Err(e) => {
                println!("Warning: Could not read '{:?}' because '{}'", folder, e);
            }
        }
    }

    let mut valid_jsons: Vec<PathBuf> = Vec::new();
    for dir_entry in valid_dir_entries {
        match dir_entry.file_type() {
            Ok(file_type) => {
                if !file_type.is_file() {
                    continue;
                }
            }
            Err(_) => {
                continue;
            }
        }

        let path: PathBuf = dir_entry.path();
        match path.extension() {
            Some(ext) => {
                if ext.to_ascii_lowercase() != "json" {
                    continue;
                }
            }
            None => {
                continue;
            }
        }
        valid_jsons.push(path);
    }
    Ok(valid_jsons)
}

fn benchmark_json_to_struct(file_path: &Path) -> Result<BenchmarkJSON> {
    let file: File = File::open(file_path)?;

    let reader: BufReader<File> = BufReader::new(file);
    let j = serde_json::from_reader(reader)?;
    Ok(j)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BenchmarkJSON {
    filepath: String,
    repeats: u64,
    memory_buffer_in_bytes: u64,
    file_size_in_bytes: u64,
    pub access_size_in_bytes: u64,
    access_pattern_in_memory: String,
    access_pattern_in_file: String,
    io_operation: String,
    prepare_file_size: bool,
    restricted_ram_in_bytes: u64,
    use_o_direct: bool,
    drop_cache_first: bool,
    reread_every_block: bool,
    delete_afterwards: bool,
    pub durations: Vec<f64>,
}

impl BenchmarkJSON {
    pub fn new_from_dir(folder: &Path) -> Result<Vec<Self>> {
        let json_paths: Vec<PathBuf> = get_all_jsons_from_directory(folder)?;
        let jsons: Result<Vec<BenchmarkJSON>> = json_paths
            .iter()
            .map(|path| benchmark_json_to_struct(path))
            .collect::<Vec<Result<BenchmarkJSON>>>()
            .into_iter()
            .collect();
        jsons
    }
    pub fn new_from_performance_benchmark(benchmark: &PerformanceBenchmark) -> Result<Vec<Self>> {
        Self::new_from_dir(&PathBuf::from(benchmark.get_benchmark_folder()))
    }
}
