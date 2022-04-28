use std::io::{self, Write};
use std::fs::{self, File};


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

const SINGLE_MODEL_HTML_PATH: &str = "single_model.html";
const SINGLE_MODEL_HTML: &str = include_str!("../frontend/single_model.html");
const MODEL_SUMMARY_HTML_PATH: &str = "model_summary.html";
const MODEL_SUMMARY_HTML: &str = include_str!("../frontend/model_summary.html");

fn create_folder_not_exists(path: &str) -> Result<(), io::Error> {
    if let Err(e) = fs::create_dir(&path) {
        match e.kind() {
            io::ErrorKind::AlreadyExists => {},
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

// I know, I know everything is hard coded
// but it works for now...
pub fn create_frontend(to_folder: &str) -> Result<(), io::Error> {
    // all folders
    let base_path = format!("{}/finished", to_folder);
    let css_path = format!("{}/css", base_path);
    let js_path = format!("{}/js", base_path);
    create_folder_not_exists(&base_path)?;
    create_folder_not_exists(&css_path)?;
    create_folder_not_exists(&js_path)?;

    // write everything
    // TODO: parametrize
    overwrite_file(NORMALIZE_CSS, &format!("{}/{}", base_path, NORMALIZE_CSS_PATH))?;
    overwrite_file(SKELETON_CSS, &format!("{}/{}", base_path, SKELETON_CSS_PATH))?;
    overwrite_file(CUSTOM_JS, &format!("{}/{}", base_path, CUSTOM_JS_PATH))?;
    overwrite_file(PLOTLY_JS, &format!("{}/{}", base_path, PLOTLY_JS_PATH))?;
    overwrite_file(SINGLE_MODEL_HTML, &format!("{}/{}", base_path, SINGLE_MODEL_HTML_PATH))?;
    overwrite_file(MODEL_SUMMARY_HTML, &format!("{}/{}", base_path, MODEL_SUMMARY_HTML_PATH))?;

    Ok(())
}
