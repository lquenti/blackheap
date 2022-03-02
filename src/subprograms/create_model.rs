use std::fs;
use std::path::PathBuf;

use crate::subprograms::helper;
use crate::benchmark_wrapper::PerformanceBenchmark;
use crate::analyzer::Analysis;

pub fn validate(model_path: &String, benchmarker_path: &String) -> Result<(), std::io::Error> {
    // The model path should be non-existing
    helper::path_does_not_exist(&PathBuf::from(model_path))?;

    // The benchmarker should obviously exist
    helper::path_exists(&PathBuf::from(benchmarker_path))?;

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
    Ok(())
}
