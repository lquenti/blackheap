// TODO: Use snitch
// TODO: Replace unwraps and excepts
use std::env;
use std::fmt;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::{Command, Stdio};

// TODO: REPLACE ME
const PATH_TO_EXECUTABLE: &str = "/home/lquenti/code/lquentin/dev/io-benchmark/build/io-benchmark.exe";

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


// TODO: Add default value to path to executable
#[derive(Debug)]
struct PerformanceBenchmark<'a> {
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

    available_ram_in_bytes: Option<i32>,
    file_path: Option<&'a str>,
}

// TODO: Why the anonymous lifetime
impl PerformanceBenchmark<'_> {
  fn new_random_uncached() -> Self {
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

        available_ram_in_bytes: None,
        file_path: None,
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
    if self.delete_afterwards {
        params.push(String::from("--delete-afterwards"));
    }
    params
  }

  fn run_test(&self, access_size: &u64) -> Result<String, String> {
    let child = Command::new(PATH_TO_EXECUTABLE)
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

fn main() {
    let random_uncached = PerformanceBenchmark::new_random_uncached();
    println!("{:?}", random_uncached);
    println!("---");

    random_uncached.run_and_save_all_benchmarks();
}
