use std::fmt;
use std::fs::{self, File};
use std::io::prelude::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::ffi::CString;

use crate::analyzer::json_reader::BenchmarkJSON;

use io_benchmarker::{self, benchmark_config_t, benchmark_results_t, access_pattern_t};

use serde::{Deserialize, Serialize};
use serde_json::json;

use anyhow::{bail, Result};

#[derive(Debug)]
pub enum AccessPattern {
    Off0,
    Rnd,
}

impl AccessPattern {
    fn to_c(&self) -> access_pattern_t {
        match self {
            Off0 => io_benchmarker::access_pattern_t_ACCESS_PATTERN_CONST,
            Rnd => io_benchmarker::access_pattern_t_ACCESS_PATTERN_RANDOM,
        }
    }
}

impl fmt::Display for AccessPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum BenchmarkType {
    RandomUncached,
    SameOffset,
}

impl FromStr for BenchmarkType {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        println!("this got called");
        match input {
            "RandomUncached" => Ok(Self::RandomUncached),
            "SameOffset" => Ok(Self::SameOffset),
            _ => Err(String::from("Unknown BenchmarkType")),
        }
    }
}

impl fmt::Display for BenchmarkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct PerformanceBenchmark<'a> {
    pub benchmark_type: BenchmarkType,

    pub is_read_op: bool,
    pub mem_pattern: AccessPattern,
    pub file_pattern: AccessPattern,
    pub repeats: u64,
    pub memory_buffer_size_in_bytes: u64,
    pub file_buffer_size_in_bytes: u64,
    pub use_o_direct: bool,
    pub drop_cache_before: bool,
    pub reread_every_block: bool,
    pub delete_afterwards: bool,

    pub model_path: &'a str,
    pub file_path: &'a str,

    pub available_ram_in_bytes: Option<u64>,
}

impl<'a> PerformanceBenchmark<'a> {
    pub fn get_all_benchmarks(
        model_path: &'a str,
        benchmark_file_path: &'a str,
        benchmarker_path: &'a str,
        root: bool,
    ) -> Vec<Self> {
        vec![
            Self::new_random_uncached_read(model_path, benchmark_file_path, benchmarker_path, root),
            Self::new_random_uncached_write(
                model_path,
                benchmark_file_path,
                benchmarker_path,
                root,
            ),
            Self::new_same_offset_read(model_path, benchmark_file_path, benchmarker_path),
            Self::new_same_offset_write(model_path, benchmark_file_path, benchmarker_path),
        ]
    }

    pub fn new_random_uncached_read(
        model_path: &'a str,
        file_path: &'a str,
        benchmarker_path: &'a str,
        root: bool,
    ) -> Self {
        PerformanceBenchmark {
            benchmark_type: BenchmarkType::RandomUncached,
            is_read_op: true,
            mem_pattern: AccessPattern::Rnd,
            file_pattern: AccessPattern::Rnd,
            repeats: 1000,
            memory_buffer_size_in_bytes: 4 * u64::pow(1024, 3),
            file_buffer_size_in_bytes: 25 * u64::pow(1024, 3),
            use_o_direct: false,
            drop_cache_before: root,
            reread_every_block: false,
            delete_afterwards: true,

            file_path,
            model_path,

            available_ram_in_bytes: None,
        }
    }

    pub fn new_random_uncached_write(
        model_path: &'a str,
        file_path: &'a str,
        benchmarker_path: &'a str,
        root: bool,
    ) -> Self {
        PerformanceBenchmark {
            benchmark_type: BenchmarkType::RandomUncached,
            is_read_op: false,
            mem_pattern: AccessPattern::Rnd,
            file_pattern: AccessPattern::Rnd,
            repeats: 1000,
            memory_buffer_size_in_bytes: 4 * u64::pow(1024, 3),
            file_buffer_size_in_bytes: 25 * u64::pow(1024, 3),
            use_o_direct: false,
            drop_cache_before: root,
            reread_every_block: false,
            delete_afterwards: true,

            file_path,
            model_path,

            available_ram_in_bytes: None,
        }
    }

    pub fn new_same_offset_read(
        model_path: &'a str,
        file_path: &'a str,
        benchmarker_path: &'a str,
    ) -> Self {
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

            file_path,
            model_path,

            available_ram_in_bytes: None,
        }
    }

    pub fn new_same_offset_write(
        model_path: &'a str,
        file_path: &'a str,
        benchmarker_path: &'a str,
    ) -> Self {
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

            file_path,
            model_path,

            available_ram_in_bytes: None,
        }
    }

    /*
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
    */

    /*
    fn run_test(&self, access_size: &u64) -> Result<String> {
        let child = Command::new(&self.benchmarker_path)
            .args(self.get_parameters(access_size))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Process could not be spawned");

        let output = child.wait_with_output().expect("failed to wait on child");

        if !output.status.success() {
            let error_message =
                std::str::from_utf8(&output.stderr).expect("Invalid UTF-8 sequence!");
            bail!(String::from(error_message));
        }

        let ret = std::str::from_utf8(&output.stdout).expect("Invalid UTF-8 sequence!");
        Ok(String::from(ret))
    }
    */

    // TODO: how small can I make the unsafe blocks?
    // TODO: BIG TODO: PORT LOGIC FROM ARG PARSER (and read through all of the C code)
    // TODO: does the CString free itself?
    fn run_test(&self, access_size: u64) -> Result<String> {
        let c_filepath = CString::new(self.file_path)?;
        let cfg = io_benchmarker::benchmark_config_t {
            filepath: c_filepath.as_ptr() as *const i8,
            memory_buffer_in_bytes: self.memory_buffer_size_in_bytes,
            file_size_in_bytes: self.file_buffer_size_in_bytes,
            access_size_in_bytes: access_size,
            number_of_io_op_tests: self.repeats,
            access_pattern_in_memory: self.mem_pattern.to_c(),
            access_pattern_in_file: self.file_pattern.to_c(),
            is_read_operation: self.is_read_op,
            prepare_file_size: true, // BIG TODO: PORT FROM ARG_PARSER
            use_o_direct: self.use_o_direct,
            drop_cache_first: self.drop_cache_before,
            do_reread: self.reread_every_block,
            delete_afterwards: self.delete_afterwards,
            restrict_free_ram_to: 0 // BIG TODO: PORT FROM ARG PARSER
        };
        let mut results: Vec<f64> = Vec::new();
        unsafe {
            // TODO: Error handling
            // TODO: Can we make it immutable by changing the C Code?
            // TODO: Inner free results
            let c_results: *mut benchmark_results_t = io_benchmarker::benchmark_file(&cfg);
            let mut i = 0;
                while i < (*c_results).length {
                    let res_time = (*c_results).durations.offset(i as isize);
                    results.push(*res_time);
                    i += 1;
                }
            std::ptr::drop_in_place(c_results);
        }
        let j = BenchmarkJSON::new_from_results(self, results, access_size);
        Ok(json!(j).to_string())
    }

    fn run_test_and_save_to_file(&self, access_size: u64, file_path: &str) {
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
        format!(
            "{}/{}/{}",
            self.model_path,
            self.benchmark_type,
            if self.is_read_op { "read" } else { "write" }
        )
    }

    pub fn run_and_save_all_benchmarks(&self) -> Result<()> {
        let benchmark_folder_path = self.get_benchmark_folder();
        fs::create_dir_all(&benchmark_folder_path)?;

        for i in 1..28 {
            let access_size = u64::pow(2, i);
            let io_type = if self.is_read_op { "read" } else { "write" };
            println!(
                "Running {} ({}) with access_size {}",
                self.benchmark_type, io_type, access_size
            );

            let path: PathBuf = [&benchmark_folder_path, &format!("{}.json", access_size)]
                .iter()
                .collect();

            // TODO
            self.run_test_and_save_to_file(access_size, path.to_str().unwrap());
        }
        Ok(())
    }
}
