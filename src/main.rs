// TODO: Remove boxes
// TODO: Use snitch
// TODO: Use real paths
use std::env;
use std::fmt;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::{Command, Stdio};

const SUBDIR_PATH: &str = "./assets/io-benchmark.exe";

#[derive(Debug)]
enum AccessPattern {
    Off0,
    Seq,
    Rnd,
}

impl fmt::Display for AccessPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccessPattern::Off0 => write!(f, "off0"),
            AccessPattern::Seq => write!(f, "seq"),
            AccessPattern::Rnd => write!(f, "rnd"),
        }
    }
}

#[derive(Debug)]
enum BenchmarkType {
    RandomUncached,
}

// TODO: Add default value to path to executable
#[derive(Debug)]
struct PerformanceBenchmark<'a> {
    path_to_executable: &'a str,
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

    available_ram_in_bytes: Option<i32>,
    file_path: Option<&'a str>,
}

// TODO: Why the anonymous lifetime
impl PerformanceBenchmark<'_> {
  fn new_random_uncached() -> Self {
    PerformanceBenchmark {
        // TODO: REPLACE ME
        path_to_executable: "/home/lquenti/code/lquentin/dev/io-benchmark/build/io-benchmark.exe",
        benchmark_type: BenchmarkType::RandomUncached,
        is_read_op: true,
        mem_pattern: AccessPattern::Rnd,
        file_pattern: AccessPattern::Rnd,
        // TODO: 1000
        repeats: 10,
        // TODO: ^3
        memory_buffer_size_in_bytes: 4 * u64::pow(1024, 2),
        file_buffer_size_in_bytes: 25 * u64::pow(1024, 2),
        use_o_direct: false,
        // TODO: True
        drop_cache_before: false,
        reread_every_block: false,

        available_ram_in_bytes: None,
        file_path: None,
    }
  }

  // TODO: Check whether &str works
  fn get_parameters(&self, access_size: &u64) -> Vec<String> {
    let mut params = vec![
        String::from(if self.is_read_op { "--read" } else { "--write" }),
        format!("--mem-pattern={}", self.mem_pattern),
        format!("--file-pattern={}", self.file_pattern),
        format!("--repeats={}", self.repeats),
        format!("--mem-buf={}", self.memory_buffer_size_in_bytes),
        format!("--file-buf={}", self.file_buffer_size_in_bytes),
        format!("--access-size={}", access_size),
    ];
    if self.use_o_direct {
        params.push(String::from("--o-direct"));
    }
    if let Some(bytes) = self.available_ram_in_bytes {
        params.push(format!("--free-ram={}", bytes));
    }
    if let Some(file_path) = self.file_path {
        params.push(format!("--file={}", file_path));
    }
    if self.drop_cache_before {
        params.push(String::from("--drop-cache"));
    }
    if self.reread_every_block {
        params.push(String::from("--reread"));
    }
    params
  }

  fn run_test(&self, access_size: &u64) -> Result<String, String> {
    let child = Command::new(self.path_to_executable)
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

  // TODO: Error handling
  fn run_test_and_save_to_file(&self, access_size: &u64, file_path: &str) -> std::io::Result<()>{
      let run_res = self.run_test(access_size);
      match run_res {
        Ok(output) => {
            let mut file = File::create(file_path)?;
            file.write_all(output.as_bytes())?;
        }
        Err(error) => {
            // TODO: Fix me
            eprintln!("Error: {}", error);
        }
      }
      Ok(())
  }

  // TODO: Error handling
  fn create_folder_in_pwd(&self) -> std::io::Result<()> {
      let cwd = env::current_dir()?;
      // TODO: FIX ME IMPORTANT
      let path: PathBuf = [cwd.to_str().unwrap(), "RandomUncached"].iter().collect();
      println!("path: {:?}", path);
      create_dir(path)?;
      Ok(())
  }

  // TODO: RENAME ME
  // TODO: Error Handling
  fn run_all_benchmarks(&self) -> std::io::Result<()> {
    self.create_folder_in_pwd()?;
    // TODO: 28
    for i in 1..5 {
        let access_size = u64::pow(2, i);
        // TODO: FIX ME IMPORTANT
        println!("Running {} with access_size {}", "RandomUncached", access_size);

        let cwd = env::current_dir()?;
        // TODO: FIX ME IMPORTANT
        // TODO: Also fix the format
        let path: PathBuf = [cwd.to_str().unwrap(), "RandomUncached", format!("{}.json", access_size).as_str()].iter().collect();
        // TODO unwrap
        self.run_test_and_save_to_file(&access_size, &path.to_str().unwrap())?;
    }
    Ok(())
  }
}

fn main() {
    let random_uncached = PerformanceBenchmark::new_random_uncached();
    println!("{:?}", random_uncached);
    println!("---");

    random_uncached.run_all_benchmarks();
}
