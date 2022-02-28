use std::fs::{canonicalize, DirEntry, File, read_dir, ReadDir};
use std::path::PathBuf;
use std::io::BufReader;

use serde::{Serialize, Deserialize};

use criterion_stats::univariate::kde::kernel::Gaussian;
use criterion_stats::univariate::kde::{Bandwidth, Kde};
use criterion_stats::univariate::Sample;

use itertools_num::linspace;

// TODO remove me after
use crate::analyzer::kde::BenchmarkKde;


fn get_all_jsons_from_directory(folder: &PathBuf) -> Vec<PathBuf> {
    let folder: PathBuf = canonicalize(&folder).unwrap();
    let dir: ReadDir = read_dir(&folder).unwrap();

    let mut valid_dir_entries: Vec<DirEntry> = Vec::new();
    for dir_entry in dir {
        match dir_entry {
            Ok(d) => { valid_dir_entries.push(d); },
            Err(e) => { println!("Warning: Could not read '{:?}' because '{}'", folder, e); }
        }
    }

    let mut valid_jsons = Vec::new();
    for dir_entry in valid_dir_entries {
        match dir_entry.file_type() {
            Ok(file_type) => {
                if !file_type.is_file() {
                    continue;
                }
            },
            Err(_) => { continue; },
        }

        let path: PathBuf = dir_entry.path();
        match path.extension() {
            Some(ext) => {
                if ext.to_ascii_lowercase() != "json" {
                    continue;
                }
            },
            None => { continue; },
        }
        valid_jsons.push(path);
    }
    valid_jsons
}

// TODO: scream louder when something goes wrong
fn benchmark_json_to_struct(file_path: &PathBuf) -> Option<BenchmarkJSON> {
    let file = File::open(file_path);

    if let Err(_) = file {
        return None;
    }
    let file = file.unwrap();
    let reader = BufReader::new(file);
    match serde_json::from_reader(reader) {
        Ok(json) => { json },
        Err(e) => { println!("{}", e); return None; },
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
    durations: Vec<f64>,
}

impl BenchmarkJSON {
    pub fn new_from_dir(folder: &PathBuf) -> Vec<Self> {
        let json_paths: Vec<PathBuf> = get_all_jsons_from_directory(&folder);
        let jsons: Vec<BenchmarkJSON> = json_paths.iter()
            .map(|path| benchmark_json_to_struct(path))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        jsons
    }

    pub fn generate_kde_from(&self, n: usize) -> BenchmarkKde {
        let slice = &self.durations[..];
        let data = Sample::new(slice);
        let kde = Kde::new(data, Gaussian, Bandwidth::Silverman);
        let h = kde.bandwidth();
        let (left, right): (f64, f64) = (data.min() - 5. * h, data.max() + 5. * h);
        let xs: Vec<f64> = linspace::<f64>(left,right, n).collect();
        let ys: Vec<f64> = kde.map(&xs).to_vec();
        BenchmarkKde { xs, ys, }
    }
}