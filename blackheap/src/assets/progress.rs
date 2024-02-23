use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use thiserror::Error;

use crate::benchmark::{Benchmark, BenchmarkScenario};

const VERSION_NUMBER: u32 = 1;
pub const FILE_NAME: &str = "BlackheapProgress.toml";

#[derive(Error, Debug)]
pub enum ProgressError {
    #[error("Serialization failed with: {0}")]
    Serialize(#[from] toml::ser::Error),

    #[error("Deserialization failed with: {0}")]
    Deserialize(#[from] toml::de::Error),

    #[error("IO failed with: {0}")]
    IO(#[from] std::io::Error),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct Meta {
    version: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Operation {
    Read,
    Write,
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Operation::Read => "read".to_string(),
            Operation::Write => "write".to_string(),
        }
    }
}

impl Operation {
    pub fn from_is_read_op(b: bool) -> Self {
        if b {
            Self::Read
        } else {
            Self::Write
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct BenchmarkStatus {
    done: bool,
    #[serde(rename = "access-sizes-done")]
    access_sizes_done: Vec<u32>,
    #[serde(rename = "access-sizes-missing")]
    access_sizes_missing: Vec<u32>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct BenchmarkProgressToml {
    meta: Meta,
    benchmarks: HashMap<BenchmarkScenario, HashMap<Operation, BenchmarkStatus>>,
}

impl BenchmarkProgressToml {
    pub fn new_from_benchmarks(benchmarks: &[Benchmark], access_sizes: &[u32]) -> Self {
        let mut benchmarks_map: HashMap<BenchmarkScenario, HashMap<Operation, BenchmarkStatus>> =
            HashMap::new();

        for benchmark in benchmarks {
            let operation = Operation::from_is_read_op(benchmark.config.is_read_operation);

            let status = BenchmarkStatus {
                done: false,
                access_sizes_done: vec![],
                access_sizes_missing: access_sizes.to_vec(),
            };

            let scenario_map = benchmarks_map.entry(benchmark.scenario).or_default();
            scenario_map.insert(operation, status);
        }

        BenchmarkProgressToml {
            meta: Meta {
                version: VERSION_NUMBER,
            },
            benchmarks: benchmarks_map,
        }
    }

    pub fn get_missing_access_sizes(&self, b: &Benchmark) -> Option<&[u32]> {
        let operation = Operation::from_is_read_op(b.config.is_read_operation);

        self.benchmarks
            .get(&b.scenario)
            .and_then(|scenario_map| scenario_map.get(&operation))
            .map(|status| status.access_sizes_missing.as_slice())
    }

    pub fn update_access_sizes_done(&mut self, b: &Benchmark, access_size: u32) {
        if let Some(operation_hashmap) = self.benchmarks.get_mut(&b.scenario) {
            let operation = Operation::from_is_read_op(b.config.is_read_operation);
            if let Some(status) = operation_hashmap.get_mut(&operation) {
                status.access_sizes_done.push(access_size);
                status
                    .access_sizes_missing
                    .retain(|&size| size != access_size);
            }
        }
    }

    pub fn to_file(&self, path: &str) -> Result<(), ProgressError> {
        let toml_str = toml::to_string(&self)?;
        fs::write(path, toml_str)?;
        Ok(())
    }

    pub fn from_file(path: &str) -> Result<Self, ProgressError> {
        let toml_str = fs::read_to_string(path)?;
        let toml: BenchmarkProgressToml = toml::from_str(&toml_str)?;
        Ok(toml)
    }
}
