pub mod json_reader;
pub mod kde;
pub mod prediction_model;

use std::fs::File;
use std::io::{BufReader, Write};

use crate::analyzer::prediction_model::Models;
use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;
use crate::analyzer::prediction_model::Interval;
use crate::benchmark_wrapper::BenchmarkType;
use crate::benchmark_wrapper::PerformanceBenchmark;
use crate::frontend;
use crate::use_model::CsvLine;

use anyhow::Result;

use serde::{self, Deserialize, Serialize};
use serde_json::json;

/**
 * Q: Why do all those <sthsth>Analysis-Structs have code duplication?
 *    Can't you just use something like Box<dyn PredictionModel>
 *    where PredictionModel is a SuperTrait inheriting all traits?
 * A: I wish I could. In order to use a trait-reference as a struct
 *    member, the trait has to be "object safe". This requires that any trait method does not have
 *    generic parameters. Sadly, this is the case for Serialize, thus we can't have any
 *    Serializable Trait as a struct member, which makes the whole analysis not serializable.
 *    So we have 2 options:
 *    1. Use type erasure in order to not have generic parameters (see: erased_serde)
 *    2. use code duplication
 *    3. Use an enum
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub benchmark_type: BenchmarkType,
    pub is_read_op: bool,
    pub kdes: Vec<BenchmarkKde>,
    pub model: Models,
}

impl Analysis {
    pub fn new_from_finished_benchmark(benchmark: PerformanceBenchmark<'_>, use_linear: bool) -> Self {
        let mut jsons: Vec<BenchmarkJSON> =
            BenchmarkJSON::new_from_performance_benchmark(&benchmark);
        jsons.sort_by_key(|j| j.access_size_in_bytes);
        let kdes: Vec<BenchmarkKde> = jsons
            .iter()
            .map(|j| BenchmarkKde::from_benchmark(j, 100))
            .collect();
        let model = match use_linear {
            true => Models::new_linear(&jsons, &kdes, Interval::new()),
            _ => Models::new_constant_linear(&jsons, &kdes, Interval::new())
        };
        Self {
            benchmark_type: benchmark.benchmark_type,
            is_read_op: benchmark.is_read_op,
            kdes,
            model,
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

    pub fn all_to_file(xs: &[Self], to_folder: &str) -> Result<()> {
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
            let approximated_time = a.model.evaluate(line.bytes);

            // if the model isn't defined on that interval, skip
            if approximated_time.is_none() {
                continue;
            }

            if approximated_time.unwrap() < line.sec {
                continue;
            }
            // do we have a upper bound already?
            res = match res {
                // if not, this is the best until now
                None => Some(a),
                // if so, lets choose the tighter bound
                Some(a2) => Some(
                    if a2.model.evaluate(line.bytes) < approximated_time {
                        a2
                    } else {
                        a
                    },
                ),
            };
        }
        res
    }
}
