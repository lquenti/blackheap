use std::fs::File;
use std::io::{self, Write};
use std::collections::HashMap;
use std::path::Path;

use lazy_static::lazy_static;

pub mod progress;

const README: &[u8; 27] = include_bytes!("../../assets/README.md");
const JUPYTER_NOTEBOOK: &[u8; 12] = include_bytes!("../../assets/AnalysisTool.ipynb");

lazy_static! {
    static ref FILES: HashMap<String, &'static [u8]> = {
        let mut map = HashMap::new();
        map.insert(String::from("README.md"), &README[..]);
        map.insert(String::from("AnalysisTool.ipynb"), &JUPYTER_NOTEBOOK[..]);
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
