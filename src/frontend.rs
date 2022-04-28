use std::fs::{self, File};
use std::io::{self, Write};

use crate::analyzer::Analysis;
use crate::benchmark_wrapper::BenchmarkType;
use crate::use_model::Report;

use anyhow::Result;

// If anyone finds a way on how to not specify the number of bytes
// while not assuming any encoding please PR.
const NORMALIZE_CSS_PATH: &str = "css/normalize.css";
const NORMALIZE_CSS: &str = include_str!("../frontend/css/normalize.css");
const SKELETON_CSS_PATH: &str = "css/skeleton.css";
const SKELETON_CSS: &str = include_str!("../frontend/css/skeleton.css");
const CUSTOM_JS_PATH: &str = "js/custom.js";
const CUSTOM_JS: &str = include_str!("../frontend/js/custom.js");
const PLOTLY_JS_PATH: &str = "js/plotly.1.33.1.min.js";
const PLOTLY_JS: &str = include_str!("../frontend/js/plotly.1.33.1.min.js");

const SINGLE_MODEL_HTML: &str = include_str!("../frontend/single_model.html");
const MODEL_SUMMARY_HTML_PATH: &str = "model_summary.html";
const MODEL_SUMMARY_HTML: &str = include_str!("../frontend/model_summary.html");

const PLACEHOLDER_BENCHMARK_TYPE: &str = "TYPE_OF_BENCHMARK";
const PLACEHOLDER_IS_READ_OP: &str = "IS_READING_OPERATION";

fn create_folder_not_exists(path: &str) -> Result<(), io::Error> {
    if let Err(e) = fs::create_dir(&path) {
        match e.kind() {
            io::ErrorKind::AlreadyExists => {}
            _ => {
                return Err(e);
            }
        }
    }
    Ok(())
}

fn overwrite_file(data: &str, path: &str) -> Result<(), io::Error> {
    let mut file = File::create(path)?;
    write!(file, "{}", data)?;
    Ok(())
}

fn parametrize_single_model(b: &BenchmarkType, is_read_op: bool) -> String {
    SINGLE_MODEL_HTML
        .replace(PLACEHOLDER_BENCHMARK_TYPE, &format!("\"{}\"", b))
        .replace(PLACEHOLDER_IS_READ_OP, &format!("{}", is_read_op))
}

// I know, I know everything is hard coded
// but it works for now...
pub fn create_frontend(xs: &[Analysis], to_folder: &str) -> Result<(), io::Error> {
    // all folders
    let base_path = format!("{}/finished", to_folder);
    let css_path = format!("{}/css", base_path);
    let js_path = format!("{}/js", base_path);
    create_folder_not_exists(&base_path)?;
    create_folder_not_exists(&css_path)?;
    create_folder_not_exists(&js_path)?;

    // write static
    overwrite_file(
        NORMALIZE_CSS,
        &format!("{}/{}", base_path, NORMALIZE_CSS_PATH),
    )?;
    overwrite_file(
        SKELETON_CSS,
        &format!("{}/{}", base_path, SKELETON_CSS_PATH),
    )?;
    overwrite_file(CUSTOM_JS, &format!("{}/{}", base_path, CUSTOM_JS_PATH))?;
    overwrite_file(PLOTLY_JS, &format!("{}/{}", base_path, PLOTLY_JS_PATH))?;

    // write summary
    overwrite_file(MODEL_SUMMARY_HTML, MODEL_SUMMARY_HTML_PATH)?;

    // write all single models, parametrized
    let all_benchmarks: Vec<(&BenchmarkType, bool)> = xs
        .iter()
        .map(|b| (&b.benchmark_type, b.is_read_op))
        .collect();
    for (benchmark_type, is_read_op) in all_benchmarks {
        let file_path = format!(
            "{}/{}_{}.html",
            base_path,
            benchmark_type,
            if is_read_op { "read" } else { "write" }
        );
        overwrite_file(
            &parametrize_single_model(benchmark_type, is_read_op),
            &file_path,
        )?;
    }

    Ok(())
}

pub fn use_frontend(report: &Report, to: &str) -> Result<()> {
    create_folder_not_exists(to)?;
    Ok(())
}
