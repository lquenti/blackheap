// TODO: Use snitch
// TODO: Replace unwraps and excepts
// TODO: Add more typing (and a linter that rejects not completely typed code.)
// TODO: Replace paths with AsRef<Path>
use std::env;
use std::fmt;
use std::fs::{canonicalize, create_dir, DirEntry, File, read_dir, ReadDir};
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use clap::{AppSettings, IntoApp, Parser, Subcommand};

use criterion_stats::univariate::kde::kernel::Gaussian;
use criterion_stats::univariate::kde::{Bandwidth, Kde};
use criterion_stats::univariate::Sample;

use itertools_num::linspace;

use serde::{Serialize, Deserialize};

const NAME: &str = "io-modeller";
const AUTHOR: &str = "Lars Quentin <lars.quentin@gwdg.de>";
const VERSION: &str = "0.1";
const ABOUT: &str = "A blackbox modeller for I/O-classification";

// TODO: some have to be cwd, some path of io-benchmarker
// Probably we should use lazy_static
const DEFAULT_MODEL_PATH: &str = "./default-model";
const DEFAULT_BENCHMARK_FILE_PATH: &str = "/tmp/io_benchmark_test_file.dat";
const DEFAULT_BENCHMARKER_PATH: &str = "./io-benchmarker.exe";

#[derive(Parser)]
#[clap(name = NAME, author = AUTHOR, version = VERSION, about = ABOUT, long_about = None)]
#[clap(global_setting(AppSettings::InferLongArgs))]
#[clap(global_setting(AppSettings::PropagateVersion))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new performance model
    CreateModel {
        /// Path to where the models will be saved.
        #[clap(short, long, default_value_t = String::from(DEFAULT_MODEL_PATH))]
        to: String,
        /// Path to where the benchmark should be done.
        #[clap(short, long, default_value_t = String::from(DEFAULT_BENCHMARK_FILE_PATH))]
        file: String,
        #[clap(short, long, default_value_t = String::from(DEFAULT_BENCHMARKER_PATH))]
        benchmarker: String,
    },
    /// Evaluate recorded I/O accesses according to previously created benchmark.
    UseModel {
        /// Path to model on which the performane will be evaluated on.
        #[clap(short, long, required = true)]
        model: String,
        /// Path to the recorded io accesses.
        #[clap(short, long, required = true)]
        file: String,
    },
}
#[derive(Debug)]
enum AccessPattern {
    Off0,
    Seq,
    Rnd,
}

impl fmt::Display for AccessPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Debug)]
enum BenchmarkType {
    RandomUncached,
}

impl fmt::Display for BenchmarkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug)]
struct PerformanceBenchmark {
    benchmark_type: BenchmarkType,

    is_read_op: bool,
    mem_pattern: AccessPattern,
    file_pattern: AccessPattern,
    repeats: u32,
    memory_buffer_size_in_bytes: u64,
    file_buffer_size_in_bytes: u64,
    use_o_direct: bool,
    drop_cache_before: bool,
    reread_every_block: bool,
    delete_afterwards: bool,

    benchmarker_path: String,
    file_path: String,

    available_ram_in_bytes: Option<i32>,
}

impl PerformanceBenchmark {
  fn new_random_uncached(benchmarker_path: &String, file_path: &String) -> Self {
    PerformanceBenchmark {
        benchmark_type: BenchmarkType::RandomUncached,
        is_read_op: true,
        mem_pattern: AccessPattern::Rnd,
        file_pattern: AccessPattern::Rnd,
        repeats: 1000,
        memory_buffer_size_in_bytes: 4 * u64::pow(1024, 3),
        file_buffer_size_in_bytes: 25 * u64::pow(1024, 3),
        use_o_direct: false,
        drop_cache_before: true,
        reread_every_block: false,
        delete_afterwards: true,

        benchmarker_path: benchmarker_path.clone(),
        file_path: file_path.clone(),

        available_ram_in_bytes: None,
    }
  }

  fn get_parameters(&self, access_size: &u64) -> Vec<String> {
    let mut params = vec![
        String::from(if self.is_read_op { "--read" } else { "--write" }),
        format!("--mem-pattern={}", self.mem_pattern),
        format!("--file-pattern={}", self.file_pattern),
        format!("--repeats={}", self.repeats),
        format!("--mem-buf={}", self.memory_buffer_size_in_bytes),
        format!("--file-buf={}", self.file_buffer_size_in_bytes),
        format!("--access-size={}", access_size),
        format!("--file={}", self.file_path),
    ];
    if self.use_o_direct {
        params.push(String::from("--o-direct"));
    }
    if let Some(bytes) = self.available_ram_in_bytes {
        params.push(format!("--free-ram={}", bytes));
    }
    if self.drop_cache_before {
        params.push(String::from("--drop-cache"));
    }
    if self.reread_every_block {
        params.push(String::from("--reread"));
    }
    if self.delete_afterwards {
        params.push(String::from("--delete-afterwards"));
    }
    params
  }

  fn run_test(&self, access_size: &u64) -> std::result::Result<String, String> {
    let child = Command::new(&self.benchmarker_path)
        .args(self.get_parameters(access_size))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Process could not be spawned");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    if !output.status.success() {
        let error_message = std::str::from_utf8(&output.stderr)
            .expect("Invalid UTF-8 sequence!");
        return Err(String::from(error_message));
    }

    let ret = std::str::from_utf8(&output.stdout)
        .expect("Invalid UTF-8 sequence!");
    Ok(String::from(ret))
  }

  fn run_test_and_save_to_file(&self, access_size: &u64, file_path: &str){
      let run_res = self.run_test(access_size);
      match run_res {
        Ok(output) => {
            let mut file = File::create(file_path).unwrap();
            file.write_all(output.as_bytes()).unwrap();
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
      }
  }

  // TODO: REWRITE ME FOR FOLDER STRUCTURE
  fn create_folder_in_pwd(&self) {
      let cwd = env::current_dir().unwrap();
      let path: PathBuf = [cwd.to_str().unwrap(), self.benchmark_type.to_string().as_str()].iter().collect();
      println!("path: {:?}", path);
      create_dir(path).unwrap();
  }

  fn run_and_save_all_benchmarks(&self) {
    let benchmark_type_string = self.benchmark_type.to_string();
    let benchmark_type_str = benchmark_type_string.as_str();

    self.create_folder_in_pwd();
    for i in 1..28 {
        let access_size = u64::pow(2, i);
        println!("Running {} with access_size {}", benchmark_type_str, access_size);

        let cwd = env::current_dir().unwrap();
        let path: PathBuf = [
            cwd.to_str().unwrap(),
            benchmark_type_str,
            format!("{}.json", access_size).as_str()
        ].iter().collect();

        self.run_test_and_save_to_file(&access_size, &path.to_str().unwrap());
    }
  }
}

// --------------------------------------
// Begin JSON

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
struct BenchmarkJSON {
    filepath: String,
    repeats: u64,
    memory_buffer_in_bytes: u64,
    file_size_in_bytes: u64,
    access_size_in_bytes: u64,
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
    fn new_from_dir(folder: &PathBuf) -> Vec<Self> {
        let json_paths: Vec<PathBuf> = get_all_jsons_from_directory(&folder);
        println!("json_paths: {:?}", json_paths);
        let jsons: Vec<BenchmarkJSON> = json_paths.iter()
            .map(|path| benchmark_json_to_struct(path))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        jsons
    }

    fn generate_kde_from(&self, n: &u64) -> BenchmarkKde {
        let slice = &self.durations[..];
        let data = Sample::new(slice);
        let kde = Kde::new(data, Gaussian, Bandwidth::Silverman);
        let h = kde.bandwidth();
        let (left, right): (f64, f64) = (data.min() - 5. * h, data.max() + 5. * h);
        let xs: Vec<f64> = linspace::<f64>(left,right, 100).collect();
        let ys: Vec<f64> = kde.map(&xs).to_vec();
        BenchmarkKde { left, right, xs, ys, }
    }
}

struct BenchmarkKde {
    left: f64,
    right: f64,
    xs: Vec<f64>,
    ys: Vec<f64>
}

// -------
// main

fn path_exists(path: &PathBuf) -> Result<(), std::io::Error> {
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{:?} does not exist!", path)
        ));
    }
    Ok(())
}

fn path_does_not_exist(path: &PathBuf) -> Result<(), std::io::Error> {
    if path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("{:?} already exists!", path),
        ));
    }
    Ok(())
}

fn validate_create_model(model_path: &String, benchmark_file_path: &String, benchmarker_path: &String) -> Result<(), std::io::Error> {
    // The model path should be a non-existing directory with an existing parent directory
    path_does_not_exist(&PathBuf::from(model_path))?;
    let mut parent = PathBuf::from(model_path);
    parent.pop();
    path_exists(&parent)?;

    // The benchmark parent should exist in order to create the file in
    let mut parent = PathBuf::from(benchmark_file_path);
    parent.pop();
    path_exists(&parent)?;

    // The benchmarker should obviously exist
    path_exists(&PathBuf::from(benchmarker_path))?;

    Ok(())
}

fn create_model(mode_path: &String, benchmark_file_path: &String, benchmarker_path: &String) {
    //let random_uncached = PerformanceBenchmark::new_random_uncached(benchmarker_path, file_path);
    // TODO
    // - [ ] (colourful) CLI plotting for progress
    // - [ ] Create folder-structure
    // - [ ] Run benchmarks with arguments provided
    // - [ ] Create KDEs
}

fn use_model() {
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::CreateModel { to, file, benchmarker } => {
            if let Err(e) = validate_create_model(to, file, benchmarker) {
                let mut app = Cli::into_app();
                app.error(
                    clap::ErrorKind::InvalidValue,
                    format!("{:?}", e)
                ).exit();
            }
            create_model(to, file, benchmarker);
        },
        Commands::UseModel { model, file } => {
        },
    }
    //BenchmarkJSON::new_from_dir(&PathBuf::from("/home/lquenti/code/io-modeller/RandomUncached"));
}
