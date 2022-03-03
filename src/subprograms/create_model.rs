use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

use crate::subprograms::helper;
use crate::benchmark_wrapper::PerformanceBenchmark;
use crate::analyzer::Analysis;
use crate::analyzer::linear_model::LinearModelJSON;

use serde_json::json;

pub fn validate(model_path: &String, benchmarker_path: &String) -> Result<(), std::io::Error> {
    // The benchmarker should obviously exist
    helper::path_exists(&PathBuf::from(benchmarker_path))?;

    Ok(())
}


// TODO move me
// TODO dont copy
fn model_to_json(analyzed: &Vec<Analysis>) -> String {
    json![
        analyzed.iter()
        .map(|a| LinearModelJSON {
            benchmark_type: a.benchmark.benchmark_type.clone(),
            is_read_op: a.benchmark.is_read_op,
            model: a.linear_model.clone(),
        }).collect::<Vec<LinearModelJSON>>()
    ].to_string()
}

fn save_analysis_model(analyzed: &Vec<Analysis>) -> Result<(), io::Error> {
    let json_str = model_to_json(analyzed);
    let path = format!("{}/LinearModel.json", analyzed[0].benchmark.model_path);
    println!("{}", path);

    let mut output = File::create(path)?;
    write!(output, "{}", json_str)?;
    Ok(())
}


pub fn create_model(model_path: &String, benchmark_file_path: &String, benchmarker_path: &String) -> Result<(), std::io::Error> {
    // create folders
    fs::create_dir_all(model_path)?;

    let mut parent = PathBuf::from(benchmark_file_path);
    parent.pop();
    fs::create_dir_all(parent)?;

    let mut analyzed: Vec<Analysis> = Vec::new();

    let all_benchmarks = PerformanceBenchmark::get_all_benchmarks(model_path, benchmark_file_path, benchmarker_path);
    for benchmark in all_benchmarks {
        // run benchmark
        benchmark.run_and_save_all_benchmarks()?;

        // Run analysis
        let res = Analysis::new_from_finished_benchmark(benchmark);
        res.save_html_report()?;
        analyzed.push(res);
    }
    save_analysis_model(&analyzed)?;

    Ok(())
}
