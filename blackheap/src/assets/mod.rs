use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use lazy_static::lazy_static;

pub mod progress;

const JUPYTER_NOTEBOOK: &[u8; 7528] = include_bytes!("../../assets/AnalysisTool.ipynb");
const BUILD_MODELS: &[u8; 13904] = include_bytes!("../../assets/build_models.py");
const GITIGNORE: &[u8; 3079] = include_bytes!("../../assets/.gitignore");
const README: &[u8; 27] = include_bytes!("../../assets/README.md");
const REQUIREMENTS: &[u8; 47] = include_bytes!("../../assets/requirements.txt");
const VERIFY: &[u8; 6379] = include_bytes!("../../assets/verify_model.py");

lazy_static! {
    static ref FILES: HashMap<String, &'static [u8]> = {
        let mut map = HashMap::new();
        map.insert(String::from("AnalysisTool.ipynb"), &JUPYTER_NOTEBOOK[..]);
        map.insert(String::from("build_models.py"), &BUILD_MODELS[..]);
        map.insert(String::from(".gitignore"), &GITIGNORE[..]);
        map.insert(String::from("README.md"), &README[..]);
        map.insert(String::from("requirements.txt"), &REQUIREMENTS[..]);
        map.insert(String::from("verify_model.py"), &VERIFY[..]);
        map
    };
}

pub fn dump_assets(dir: &Path) -> io::Result<()> {
    for (filename, bytes) in FILES.iter() {
        let file_path = dir.join(filename);

        let mut file = File::create(file_path)?;
        file.write_all(bytes)?;
    }
    Ok(())
}
