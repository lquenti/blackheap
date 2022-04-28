pub mod json_reader;
pub mod kde;
pub mod linear_model;

use std::fs::File;
use std::io::{self, Write};

use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;
use crate::analyzer::linear_model::LinearModel;
use crate::benchmark_wrapper::BenchmarkType;
use crate::benchmark_wrapper::PerformanceBenchmark;
use crate::frontend;
use crate::use_model::CsvLine;

use serde::{Deserialize, Serialize};
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

    pub fn all_to_json(xs: &Vec<Self>) -> String {
        json![xs].to_string()
    }

    pub fn all_to_file(xs: &Vec<Self>, to_folder: &str) -> Result<(), io::Error> {
        let path = format!("{}/finished", to_folder);
        frontend::create_frontend(xs, to_folder)?;
        // write file
        let mut output = File::create(format!("{}/Model.json", path))?;
        write!(output, "{}", Self::all_to_json(xs))?;

        Ok(())
    }

    pub fn find_lowest_upper_bound(xs: Vec<Self>, line: &CsvLine) -> Option<&LinearModel> {
        // TODO: from LinearModels, recode me
        /*
        pub fn find_lowest_upper_bound(&self, line: &CsvLine) -> Option<&LinearModelJSON> {
            let mut res = None;
            for lm in self.0.iter() {
                // Apples and oranges
                if lm.is_read_op != (line.io_type == 'r') {
                    continue;
                }
                println!("{:?}", lm);

                let approximated_time = lm.model.evaluate(line.bytes);
                println!("{:?} -> {}", lm, approximated_time);

                // we are looking for an upper bound. Thus if it is lower, we can instantly reject it.
                if approximated_time < line.sec {
                    continue;
                }

                // do we have a upper bound already?
                res = match res {
                    // if not, this is the best until now
                    None => Some(lm),
                    // if so, lets choose the tighter bound
                    Some(lm2) => Some(if lm2.model.evaluate(line.bytes) < approximated_time {
                        lm2
                    } else {
                        lm
                    }),
                }
            }
            res
        }
            */
        None
    }
}
