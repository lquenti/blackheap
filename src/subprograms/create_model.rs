use std::fs::{create_dir, create_dir_all, File};
use std::path::PathBuf;

use crate::subprograms::helper::{path_exists, path_does_not_exist};
use crate::benchmark_wrapper::PerformanceBenchmark;
use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;
use crate::analyzer::linear_model::LinearModel;
use crate::html_templater::ResultTemplate;

// TODO this one shouldnt be here
use sailfish::TemplateOnce;

pub fn validate_create_model(model_path: &String, benchmarker_path: &String) -> Result<(), std::io::Error> {
    // The model path should be non-existing
    //TODO
    path_does_not_exist(&PathBuf::from(model_path))?;

    // The benchmarker should obviously exist
    //TODO
    path_exists(&PathBuf::from(benchmarker_path))?;

    Ok(())
}





pub fn create_model(model_path: &String, benchmark_file_path: &String, benchmarker_path: &String) -> Result<(), std::io::Error> {
    // create folders
    create_dir_all(model_path)?;

    let mut parent = PathBuf::from(benchmark_file_path);
    parent.pop();
    create_dir_all(parent)?;

    // Create Benchmarks
    let random_uncached = PerformanceBenchmark::new_random_uncached(benchmarker_path, benchmark_file_path);
    random_uncached.run_and_save_all_benchmarks(model_path)?;

    // re-read benchmarks
    let benchmark_folder = random_uncached.get_benchmark_folder(model_path);
    let mut jsons = BenchmarkJSON::new_from_dir(&PathBuf::from(benchmark_folder));
    jsons.sort_by_key(|j| j.access_size_in_bytes);

    // Generate KDEs
    let kdes: Vec<BenchmarkKde> = jsons.iter().map(|j| j.generate_kde_from(100)).collect();
    let jsons_kdes: Vec<(&BenchmarkJSON, &BenchmarkKde)> = jsons.iter().zip(kdes.iter()).collect();

    // Create linear model
    let linear_model = LinearModel::from_jsons_kdes(&jsons, &kdes);
    let linear_model_svg = linear_model.to_svg(&jsons, &kdes);

    // save linear model
    let mut model_file = File::create(format!("{}/LinearModel.json", model_path))?;
    write!(model_file, "{}", linear_model.to_json())?;

    // Generate HTML report
    let ctx = ResultTemplate {
        benchmark_name: random_uncached.benchmark_type.to_string(),
        jsons_kdes,
        linear_model,
        linear_model_svg,
    };
    let html: String = ctx.render_once().unwrap();

    let html_template_path = format!("{}/{}", model_path, String::from("html"));
    create_dir(&html_template_path)?;

    let mut output = File::create(format!("{}/{}.html", &html_template_path, random_uncached.benchmark_type.to_string()))?;
    write!(output, "{}", html)?;

    Ok(())
}
