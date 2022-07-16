use std::fs::{self, DirEntry, File, ReadDir};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::benchmark_wrapper::PerformanceBenchmark;

use serde::{Deserialize, Serialize};

fn get_all_jsons_from_directory(folder: &Path) -> Vec<PathBuf> {
    let folder: PathBuf = fs::canonicalize(&folder).unwrap();
    let dir: ReadDir = fs::read_dir(&folder).unwrap();

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
    valid_jsons
}

// TODO: scream louder when something goes wrong
// TODO: ANYHOW HERE
fn benchmark_json_to_struct(file_path: &Path) -> Option<BenchmarkJSON> {
    let file: Result<File, std::io::Error> = File::open(file_path);

    if file.is_err() {
        return None;
    }
    let file: File = file.unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    match serde_json::from_reader(reader) {
        Ok(json) => json,
        Err(e) => {
            println!("{}", e);
            None
        }
    }
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
    pub fn new_from_dir(folder: &Path) -> Vec<Self> {
        let json_paths: Vec<PathBuf> = get_all_jsons_from_directory(folder);
        let jsons: Vec<BenchmarkJSON> = json_paths
            .iter()
            .filter_map(|path| benchmark_json_to_struct(path))
            .collect();
        jsons
    }
    pub fn new_from_performance_benchmark(benchmark: &PerformanceBenchmark) -> Vec<Self> {
        Self::new_from_dir(&PathBuf::from(benchmark.get_benchmark_folder()))
    }
}
