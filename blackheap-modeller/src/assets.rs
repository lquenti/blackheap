// If they aren't there: Generate them by running the main Makefile
const BENCHMARKER: &[u8] = include_bytes!("../assets/blackheap-benchmark.exe");
const FRONTEND: &str = include_str!("../assets/index.html");

use std::fs::{self, File, Permissions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

use anyhow::Result;

pub fn dump_static_files(to: &str) -> Result<()> {
    let benchmarker_path: String = format!("{}/blackheap-benchmark.exe", to);
    let mut benchmarker: File = File::create(&benchmarker_path)?;
    benchmarker.write_all(BENCHMARKER)?;
    fs::set_permissions(&benchmarker_path, Permissions::from_mode(0o555))?;

    let mut frontend: File = File::create(format!("{}/index.html", to))?;
    write!(frontend, "{}", FRONTEND)?;
    Ok(())
}
