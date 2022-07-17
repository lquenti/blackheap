pub mod json_reader;
pub mod kde;
pub mod prediction_model;

use std::fs::File;
use std::io::Write;

use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;
use crate::analyzer::prediction_model::Interval;
use crate::analyzer::prediction_model::Models;
use crate::benchmark_wrapper::BenchmarkType;
use crate::benchmark_wrapper::PerformanceBenchmark;

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
    pub fn new_from_finished_benchmark(
        benchmark: PerformanceBenchmark<'_>,
        use_linear: bool,
    ) -> Result<Self> {
        let mut jsons: Vec<BenchmarkJSON> =
            BenchmarkJSON::new_from_performance_benchmark(&benchmark)?;
        jsons.sort_by_key(|j| j.access_size_in_bytes);
        let kdes: Vec<BenchmarkKde> = jsons
            .iter()
            .map(|j| BenchmarkKde::from_benchmark(j, 100))
            .collect();
        let model = match use_linear {
            true => Models::new_linear(&jsons, &kdes, Interval::new()),
            _ => Models::new_constant_linear(&jsons, &kdes, Interval::new()),
        };
        Ok(Self {
            benchmark_type: benchmark.benchmark_type,
            is_read_op: benchmark.is_read_op,
            kdes,
            model,
        })
    }

    pub fn all_to_json(xs: &[Self]) -> String {
        json![xs].to_string()
    }

    pub fn all_to_file(xs: &[Self], to_folder: &str) -> Result<()> {
        // write file
        let mut output: File = File::create(format!("{}/Model.json", to_folder))?;
        write!(output, "{}", Self::all_to_json(xs))?;

        Ok(())
    }
}
