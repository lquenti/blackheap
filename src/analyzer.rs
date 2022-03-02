pub mod json_reader;
pub mod kde;
pub mod linear_model;

use std::path::Path;
use std::io;
use std::fs::{self, File};

use crate::benchmark_wrapper::PerformanceBenchmark;
use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;
use crate::analyzer::linear_model::LinearModel;

pub struct Analysis {
    benchmark: PerformanceBenchmark,
    jsons: Vec<BenchmarkJSON>,
    kdes: Vec<BenchmarkKde>,
    linear_model: LinearModel,
}

impl Analysis {
    #[allow(dead_code)]
    fn new_from_dir<T: AsRef<Path>>(_path: T) {
        panic!("Not yet implemented");
    }

    pub fn new_from_finished_benchmark(benchmark: PerformanceBenchmark) -> Self {
        let mut jsons: Vec<BenchmarkJSON> = BenchmarkJSON::new_from_performance_benchmark(&benchmark);
        jsons.sort_by_key(|j| j.access_size_in_bytes);
        let kdes: Vec<BenchmarkKde> = jsons.iter().map(|j| BenchmarkKde::from_benchmark(j, 100)).collect();
        let linear_model = LinearModel::from_jsons_kdes(&jsons, &kdes);
        Analysis { benchmark, jsons, kdes, linear_model }
    }

    pub fn create_html_report(&self) -> String {
        String::new()
    }

    pub fn save_html_report(&self) -> Result<(), io::Error> {
        let html_report = self.create_html_report();
        let html_template_path = format!("{}/{}", self.benchmark.model_path, String::from("html"));

        // A previous Analysis could have already created it.
        if let Err(e) = fs::create_dir(&html_template_path) {
            match e.kind() {
                io::ErrorKind::AlreadyExists => { },
                _ => { return Err(e); }
            }
        }

        let mut output = File::create(
            format!(
                "{}/{}_{}.html",
                &html_template_path,
                self.benchmark.benchmark_type.to_string(),
                if self.benchmark.is_read_op { "read" } else { "write" }
            )
        );
        write!(output, "{}", html_report)?;

        Ok(())
    }
}
