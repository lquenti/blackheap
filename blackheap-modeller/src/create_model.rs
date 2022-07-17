use std::fs;
use std::path::PathBuf;

use crate::analyzer::Analysis;
use crate::benchmark_wrapper::PerformanceBenchmark;
use crate::cli::ModelEnum;
use crate::assets;

use anyhow::{bail, Result};

pub fn create_model(
    model_path: &str,
    benchmark_file_path: &str,
    benchmarker_path: &str,
    root: bool,
    analyze_only: bool,
    model: ModelEnum,
) -> Result<()> {
    // create folders
    fs::create_dir_all(model_path)?;
    let use_linear: bool = ModelEnum::Linear == model;

    let mut parent: PathBuf = PathBuf::from(benchmark_file_path);
    parent.pop();
    fs::create_dir_all(parent)?;

    // dump benchmarker and frontend
    assets::dump_static_files(model_path)?;

    /*
    // begin the benchmarkey stuff
    let mut analyzed: Vec<Analysis> = Vec::new();

    let all_benchmarks: Vec<PerformanceBenchmark> = PerformanceBenchmark::get_all_benchmarks(
        model_path,
        benchmark_file_path,
        benchmarker_path,
        root,
    );
    for benchmark in all_benchmarks {
        // run benchmark
        if !analyze_only {
            benchmark.run_and_save_all_benchmarks()?;
        }

        // Run analysis
        let res: Analysis = Analysis::new_from_finished_benchmark(benchmark, use_linear)?;
        analyzed.push(res);
    }

    // remove folder if exists
    if analyze_only {
        if let Err(e) = fs::remove_dir_all(format!("{}/finished", &model_path)) {
            match e.kind() {
                std::io::ErrorKind::NotFound => {}
                _ => bail!(e),
            }
        }
    }
    Analysis::all_to_file(&analyzed, model_path)?;
    */
    Ok(())
}
