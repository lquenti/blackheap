use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use thiserror::Error;

use crate::{BenchmarkScenario, Benchmark};

const VERSION_NUMBER: u32 = 1;

#[derive(Error, Debug)]
pub enum ProgressError {
    #[error("Serialization failed with: {0}")]
    SerializeError(#[from] toml::ser::Error),

    #[error("Deserialization failed with: {0}")]
    DeserializeError(#[from] toml::de::Error),


    #[error("IO failed with: {0}")]
    IOError(#[from] std::io::Error),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Meta {
    version: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
enum Operation {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct BenchmarkStatus {
    done: bool,
    #[serde(rename = "access-sizes-done")]
    access_sizes_done: Vec<u32>,
    #[serde(rename = "access-sizes-missing")]
    access_sizes_missing: Vec<u32>,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct BenchmarkProgressToml {
    meta: Meta,
    benchmarks: HashMap<BenchmarkScenario, HashMap<Operation, BenchmarkStatus>>,
}


impl BenchmarkProgressToml {
    pub fn new_from_benchmarks(benchmarks: &[Benchmark], access_sizes: &[u32]) -> Self {
        let mut benchmarks_map: HashMap<BenchmarkScenario, HashMap<Operation, BenchmarkStatus>> = HashMap::new();

        for benchmark in benchmarks {
            let operation = if benchmark.config.is_read_operation {
                Operation::Read
            } else {
                Operation::Write
            };

            let status = BenchmarkStatus {
                done: false,
                access_sizes_done: vec![],
                access_sizes_missing: access_sizes.to_vec(),
            };

            let scenario_map = benchmarks_map.entry(benchmark.scenario).or_insert_with(HashMap::new);
            scenario_map.insert(operation, status);
        }

        BenchmarkProgressToml {
            meta: Meta { version: VERSION_NUMBER },
            benchmarks: benchmarks_map,
        }
    }

    pub fn get_missing_access_sizes(&self, b: &Benchmark) -> Option<&[u32]> {
        let operation = match b.config.is_read_operation {
            True => Operation::Read,
            False => Operation::Write,
        };

        self.benchmarks.get(&b.scenario)
            .and_then(|scenario_map| scenario_map.get(&operation))
            .map(|status| status.access_sizes_missing.as_slice())
    }


    pub fn to_file(&self, path: &str) -> Result<(), ProgressError> {
        let toml_str = toml::to_string(&self)?;
        fs::write(path, &toml_str)?;
        Ok(())
    }

    pub fn from_file(path: &str) -> Result<Self, ProgressError> {
        let toml_str = fs::read_to_string(path)?;
        let toml: BenchmarkProgressToml = toml::from_str(&toml_str)?;
        Ok(toml)
    }
}
