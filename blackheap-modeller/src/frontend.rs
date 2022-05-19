use std::fs::{self, File};
use std::io::{self, Write};

use crate::analyzer::Analysis;
use crate::benchmark_wrapper::BenchmarkType;

use anyhow::{bail, Result};

const NORMALIZE_CSS_PATH: &str = "css/normalize.css";
const NORMALIZE_CSS: &str = include_str!("../frontend/css/normalize.css");
const SKELETON_CSS_PATH: &str = "css/skeleton.css";
const SKELETON_CSS: &str = include_str!("../frontend/css/skeleton.css");
const CUSTOM_JS_PATH: &str = "js/custom.js";
const CUSTOM_JS: &str = include_str!("../frontend/js/custom.js");
const PLOTLY_JS_PATH: &str = "js/plotly.1.33.1.min.js";
const PLOTLY_JS: &str = include_str!("../frontend/js/plotly.1.33.1.min.js");
const LODASH_JS_PATH: &str = "js/lodash.4.17.21.min.js";
const LODASH_JS: &str = include_str!("../frontend/js/lodash.4.17.21.min.js");

const SINGLE_MODEL_HTML: &str = include_str!("../frontend/single_model.html");
const MODEL_SUMMARY_HTML_PATH: &str = "model_summary.html";
const MODEL_SUMMARY_HTML: &str = include_str!("../frontend/model_summary.html");

const USE_MODEL_REPORT: &str = include_str!("../frontend/use_model_report.html");
const USE_MODEL_REPORT_PATH: &str = "use_model_report.html";

const PLACEHOLDER_BENCHMARK_TYPE: &str = "TYPE_OF_BENCHMARK";
const PLACEHOLDER_IS_READ_OP: &str = "IS_READING_OPERATION";

fn create_folder_not_exists(path: &str) -> Result<()> {
    if let Err(e) = fs::create_dir(&path) {
        match e.kind() {
            io::ErrorKind::AlreadyExists => {}
            _ => {
                bail!(e);
            }
        }
    }
    Ok(())
}

fn overwrite_file(data: &str, path: &str) -> Result<()> {
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
pub fn create_frontend(xs: &[Analysis], to_folder: &str) -> Result<()> {
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
    overwrite_file(LODASH_JS, &format!("{}/{}", base_path, LODASH_JS_PATH))?;

    // write summary
    overwrite_file(
        MODEL_SUMMARY_HTML,
        &format! {"{}/{}", base_path, MODEL_SUMMARY_HTML_PATH},
    )?;

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

// TODO remove redundancy
pub fn use_frontend(to: &str) -> Result<()> {
    // all folders
    let css_path = format!("{}/css", to);
    let js_path = format!("{}/js", to);
    create_folder_not_exists(to)?;
    create_folder_not_exists(&css_path)?;
    create_folder_not_exists(&js_path)?;

    // write static
    overwrite_file(NORMALIZE_CSS, &format!("{}/{}", to, NORMALIZE_CSS_PATH))?;
    overwrite_file(SKELETON_CSS, &format!("{}/{}", to, SKELETON_CSS_PATH))?;
    overwrite_file(CUSTOM_JS, &format!("{}/{}", to, CUSTOM_JS_PATH))?;
    overwrite_file(PLOTLY_JS, &format!("{}/{}", to, PLOTLY_JS_PATH))?;

    overwrite_file(
        USE_MODEL_REPORT,
        &format!("{}/{}", to, USE_MODEL_REPORT_PATH),
    )?;

    Ok(())
}
