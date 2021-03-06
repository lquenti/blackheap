use std::fmt;
use std::fs::{self, File};
use std::io::prelude::Write;
use std::path::PathBuf;
use std::process::{Child, Command, Output, Stdio};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use anyhow::{bail, Result};

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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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
    mem_pattern: AccessPattern,
    file_pattern: AccessPattern,
    repeats: u32,
    memory_buffer_size_in_bytes: u64,
    file_buffer_size_in_bytes: u64,
    use_o_direct: bool,
    drop_cache_before: bool,
    reread_every_block: bool,
    delete_afterwards: bool,

    pub model_path: &'a str,
    file_path: &'a str,
    benchmarker_path: &'a str,

    available_ram_in_bytes: Option<i32>,
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

            benchmarker_path,
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

            benchmarker_path,
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

            benchmarker_path,
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

            benchmarker_path,
            file_path,
            model_path,

            available_ram_in_bytes: None,
        }
    }

    fn get_parameters(&self, access_size: &u64) -> Vec<String> {
        let mut params: Vec<String> = vec![
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

    fn run_test(&self, access_size: &u64) -> Result<String> {
        let child: Child = Command::new(&self.benchmarker_path)
            .args(self.get_parameters(access_size))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let output: Output = child.wait_with_output()?;

        if !output.status.success() {
            let error_message: &str = std::str::from_utf8(&output.stderr)?;
            bail!(String::from(error_message));
        }

        let ret: &str = std::str::from_utf8(&output.stdout)?;
        Ok(String::from(ret))
    }

    fn run_test_and_save_to_file(&self, access_size: &u64, file_path: &str) -> Result<()> {
        let output: String = self.run_test(access_size)?;
        let mut file = File::create(file_path)?;
        file.write_all(output.as_bytes())?;
        Ok(())
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
        let benchmark_folder_path: String = self.get_benchmark_folder();
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

            self.run_test_and_save_to_file(&access_size, path.to_str().unwrap())?;
        }
        Ok(())
    }
}
