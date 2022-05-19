use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

use crate::analyzer::Analysis;
use crate::benchmark_wrapper::BenchmarkType;
use crate::frontend;

use serde::{Deserialize, Serialize};
use serde_json::json;

use anyhow::Result;

// TODO move me somewhere else
#[derive(Debug, Deserialize)]
pub struct CsvLine {
    pub io_type: char,
    pub bytes: u64,
    pub sec: f64,
}

impl CsvLine {
    fn from_file(file: &str) -> Result<Vec<CsvLine>> {
        let mut rdr = csv::Reader::from_path(file)?;
        let mut res = Vec::new();
        for result in rdr.deserialize::<CsvLine>() {
            let record = result?;
            res.push(record);
        }
        Ok(res)
    }
}

// TODO: refactor me
#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    read_bytes_sec: Vec<(u64, f64)>,
    write_bytes_sec: Vec<(u64, f64)>,
    // key in json has to be string TODO
    number_of_classified: BTreeMap<String, u64>,
    number_of_unclassified: u64,
    // TODO: too bloaty
    model: Vec<Analysis>,
}

impl Report {
    fn key_to_string(b: BenchmarkType, r: bool) -> String {
        format!("{} {}", if r { "read" } else { "write" }, b)
    }

    fn from_measurements(model: Vec<Analysis>, measurements: &[CsvLine]) -> Self {
        let mut read_bytes_sec = Vec::new();
        let mut write_bytes_sec = Vec::new();
        let mut number_of_classified = BTreeMap::new();
        let mut number_of_unclassified = 0;

        for m in measurements {
            // TODO error detection
            // Add to total list for plots
            match m.io_type {
                'r' => read_bytes_sec.push((m.bytes, m.sec)),
                _ => write_bytes_sec.push((m.bytes, m.sec)),
            };

            // Add to either classified or unclassified
            let a = Analysis::find_lowest_upper_bound(&model, m);
            match &a {
                Some(res) => {
                    let x = number_of_classified
                        .entry(Self::key_to_string(
                            res.benchmark_type.clone(),
                            res.is_read_op,
                        ))
                        .or_insert(0);
                    *x += 1;
                }
                None => number_of_unclassified += 1,
            };
        }
        Self {
            model,
            read_bytes_sec,
            write_bytes_sec,
            number_of_classified,
            number_of_unclassified,
        }
    }

    fn save_frontend(&self, to: &str) -> Result<()> {
        frontend::use_frontend(to)?;
        // write file
        let mut output = File::create(format!("{}/Report.json", to))?;
        write!(output, "{}", json![&self])?;
        Ok(())
    }
}

pub fn use_model(model: &str, file: &str, to: &str) -> Result<()> {
    // TODO: validate

    // get measurements
    let measurements: Vec<CsvLine> = CsvLine::from_file(file)?;

    // load Analyzed
    let analyzed = Analysis::load_from_file(model)?;

    // DEBUG
    for m in &measurements {
        let oa = Analysis::find_lowest_upper_bound(&analyzed, m);
        println!(
            "{}: {} bytes in {} took less than {} ({} {})",
            if m.io_type == 'r' { "read" } else { "write" },
            m.bytes,
            m.sec,
            match &oa {
                None => String::from("<NONE>"),
                Some(a) => a
                    .model
                    .evaluate(m.bytes)
                    .unwrap_or(0.0f64)
                    .to_string(),
            },
            match &oa {
                None => String::from(""),
                Some(a) => format!("{}", a.benchmark_type),
            },
            match &oa {
                None => String::from(""),
                Some(a) => String::from(if a.is_read_op { "read" } else { "write" }),
            },
        );
        println!("----------");
    }

    let output = Report::from_measurements(analyzed, &measurements);
    output.save_frontend(to)?;

    Ok(())
}
