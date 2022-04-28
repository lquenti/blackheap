pub mod json_reader;
pub mod kde;
pub mod linear_model;

use std::io::{self, Write};
use std::fs::{self, File};

use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;
use crate::benchmark_wrapper::BenchmarkType;
use crate::analyzer::linear_model::LinearModel;
use crate::benchmark_wrapper::PerformanceBenchmark;

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    benchmark_type: BenchmarkType,
    is_read_op: bool,
    kdes: Vec<BenchmarkKde>,
    linear_model: LinearModel,
}

impl Analysis {
    pub fn new_from_finished_benchmark(benchmark: PerformanceBenchmark<'_>) -> Self {
        let mut jsons: Vec<BenchmarkJSON> =
            BenchmarkJSON::new_from_performance_benchmark(&benchmark);
        jsons.sort_by_key(|j| j.access_size_in_bytes);
        let kdes: Vec<BenchmarkKde> = jsons
            .iter()
            .map(|j| BenchmarkKde::from_benchmark(j, 100))
            .collect();
        let linear_model = LinearModel::from_jsons_kdes(&jsons, &kdes);
        Self {
            benchmark_type: benchmark.benchmark_type,
            is_read_op: benchmark.is_read_op,
            kdes,
            linear_model
        }
    }

    pub fn all_to_json(xs: &Vec<Self>) -> String {
        json![xs].to_string()
    }

    pub fn all_to_file(xs: &Vec<Self>, to_folder: &str) -> Result<(), io::Error> {
        // A previous Analysis could have already created it.
        if let Err(e) = fs::create_dir(to_folder) {
            match e.kind() {
                io::ErrorKind::AlreadyExists => {}
                _ => {
                    return Err(e);
                }
            }
        }

        // write file
        let mut output = File::create(format!("{}/Model.json", to_folder))?;
        write!(output, "{}", Self::all_to_json(xs))?;

        Ok(())
    }
}
