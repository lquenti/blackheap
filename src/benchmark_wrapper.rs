use std::fmt;
use std::process::{Command, Stdio};
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::prelude::Write;

#[derive(Debug)]
enum AccessPattern {
    Off0,
    Rnd,
}

impl fmt::Display for AccessPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Debug)]
pub enum BenchmarkType {
    RandomUncached,
    SameOffset,
}

impl fmt::Display for BenchmarkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug)]
pub struct PerformanceBenchmark {
    pub benchmark_type: BenchmarkType,

    pub is_read_op: bool,
    mem_pattern: AccessPattern,
    file_pattern: AccessPattern,
    repeats: u32,
    memory_buffer_size_in_bytes: u64,
    file_buffer_size_in_bytes: u64,
    use_o_direct: bool,
    drop_cache_before: bool,
    reread_every_block: bool,
    delete_afterwards: bool,

    pub model_path: String,
    file_path: String,
    benchmarker_path: String,

    available_ram_in_bytes: Option<i32>,
}

impl PerformanceBenchmark {
  pub fn get_all_benchmarks(model_path: &String, benchmark_file_path: &String, benchmarker_path: &String) -> Vec<Self> {
    let all_benchmarks: Vec<fn(&String, &String, &String) -> Self> = vec![
        Self::new_random_uncached_read,
        Self::new_random_uncached_write,
        Self::new_same_offset_read,
        Self::new_same_offset_write,
    ];
    all_benchmarks.iter().map(|f| f(model_path, benchmark_file_path, benchmarker_path)).collect()
  }

  pub fn new_random_uncached_read(model_path: &String, benchmark_file_path: &String, benchmarker_path: &String) -> Self {
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
        file_path: benchmark_file_path.clone(),
        model_path: model_path.clone(),

        available_ram_in_bytes: None,
    }
  }

  pub fn new_random_uncached_write(model_path: &String, benchmark_file_path: &String, benchmarker_path: &String) -> Self {
    PerformanceBenchmark {
        benchmark_type: BenchmarkType::RandomUncached,
        is_read_op: false,
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
        file_path: benchmark_file_path.clone(),
        model_path: model_path.clone(),

        available_ram_in_bytes: None,
    }
  }

  pub fn new_same_offset_read(model_path: &String, benchmark_file_path: &String, benchmarker_path: &String) -> Self {
    PerformanceBenchmark {
        benchmark_type: BenchmarkType::SameOffset,
        is_read_op: true,
        mem_pattern: AccessPattern::Off0,
        file_pattern: AccessPattern::Off0,
        repeats: 1000,
        memory_buffer_size_in_bytes: 4 * u64::pow(1024, 3),
        file_buffer_size_in_bytes: 25 * u64::pow(1024, 3),
        use_o_direct: false,
        drop_cache_before: false,
        reread_every_block: true,
        delete_afterwards: false,

        benchmarker_path: benchmarker_path.clone(),
        file_path: benchmark_file_path.clone(),
        model_path: model_path.clone(),

        available_ram_in_bytes: None,
    }
  }

  pub fn new_same_offset_write(model_path: &String, benchmark_file_path: &String, benchmarker_path: &String) -> Self {
    PerformanceBenchmark {
        benchmark_type: BenchmarkType::SameOffset,
        is_read_op: false,
        mem_pattern: AccessPattern::Off0,
        file_pattern: AccessPattern::Off0,
        repeats: 1000,
        memory_buffer_size_in_bytes: 4 * u64::pow(1024, 3),
        file_buffer_size_in_bytes: 25 * u64::pow(1024, 3),
        use_o_direct: false,
        drop_cache_before: false,
        reread_every_block: true,
        delete_afterwards: false,

        benchmarker_path: benchmarker_path.clone(),
        file_path: benchmark_file_path.clone(),
        model_path: model_path.clone(),

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

  pub fn get_benchmark_folder(&self) -> String {
    format!("{}/{}/{}", self.model_path, self.benchmark_type.to_string(), if self.is_read_op {"read"} else {"write"})
  }



  pub fn run_and_save_all_benchmarks(&self) -> Result<(), std::io::Error> {
    let benchmark_folder_path = self.get_benchmark_folder();
    fs::create_dir_all(&benchmark_folder_path)?;

    for i in 1..28 {
        let access_size = u64::pow(2, i);
        let io_type = if self.is_read_op { "read" } else { "write" };
        println!("Running {} ({}) with access_size {}", self.benchmark_type.to_string(), io_type, access_size);

        let path: PathBuf = [
            &benchmark_folder_path,
            &format!("{}.json", access_size)
        ].iter().collect();

        self.run_test_and_save_to_file(&access_size, &path.to_str().unwrap());
    }
    Ok(())
  }
}
