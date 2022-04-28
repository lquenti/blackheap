pub mod json_reader;
pub mod kde;
pub mod linear_model;

use std::fs::File;
use std::io::{self, Write, BufReader};
use std::str::FromStr;

use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;
use crate::analyzer::linear_model::LinearModel;
use crate::benchmark_wrapper::BenchmarkType;
use crate::benchmark_wrapper::PerformanceBenchmark;
use crate::frontend;
use crate::use_model::CsvLine;

// TODO: use everywhere
use anyhow::Result;

use serde::{self, de::Error, Deserialize, Deserializer, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub benchmark_type: BenchmarkType,
    pub is_read_op: bool,
    pub kdes: Vec<BenchmarkKde>,
    pub linear_model: LinearModel,
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
            linear_model,
        }
    }

    pub fn load_from_file(file_path: &str) -> Result<Vec<Self>> {
        let file = File::open(file_path)?;

        let reader = BufReader::new(file);
        let res = serde_json::from_reader(reader)?;
        Ok(res)
    }

    pub fn all_to_json(xs: &[Self]) -> String {
        json![xs].to_string()
    }

    pub fn all_to_file(xs: &[Self], to_folder: &str) -> Result<(), io::Error> {
        let path = format!("{}/finished", to_folder);
        frontend::create_frontend(xs, to_folder)?;
        // write file
        let mut output = File::create(format!("{}/Model.json", path))?;
        write!(output, "{}", Self::all_to_json(xs))?;

        Ok(())
    }

    pub fn find_lowest_upper_bound<'a>(xs: &'a [Self], line: &'a CsvLine) -> Option<&'a Self> {
        let mut res = None;
        for a in xs.iter() {
            if a.is_read_op != (line.io_type == 'r') {
                continue;
            }
            let approximated_time = a.linear_model.evaluate(line.bytes);

            if approximated_time < line.sec {
                continue;
            }
            // do we have a upper bound already?
            res = match res {
                // if not, this is the best until now
                None => Some(a),
                // if so, lets choose the tighter bound
                Some(a2) => Some(if a2.linear_model.evaluate(line.bytes) < approximated_time {
                    a2
                } else {
                    a
                }),
            };
        }
        res
    }
}
