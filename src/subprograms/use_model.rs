use std::path::PathBuf;

use crate::benchmark_wrapper::BenchmarkType;
use crate::analyzer::linear_model::LinearModels;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CsvLine {
    pub io_type: char,
    pub bytes: u64,
    pub sec: f64
}

impl CsvLine {
    fn from_file(file: &String) -> Result<Vec<CsvLine>, std::io::Error> {
        let mut rdr = csv::Reader::from_path(file)?;
        let mut res = Vec::new();
        for result in rdr.deserialize::<CsvLine>() {
            let record = result?;
            res.push(record);
        }
        Ok(res)
    }
}

pub fn use_model(model: &String, file: &String) -> Result<(), std::io::Error> {
    // TODO: validate

    // get measurements
    let measurements: Vec<CsvLine> = CsvLine::from_file(file)?;
    for m in measurements {
        println!("{:?}", m);
    }

    // load LinearModel
    let model = LinearModels::from_file(&PathBuf::from(model));
    println!("{:?}", model.unwrap());


    Ok(())
}
