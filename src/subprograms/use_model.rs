use crate::analyzer::linear_model::LinearModel;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CsvLine {
    io_type: char,
    bytes: u64,
    sec: f64
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
    // get model
    let model: LinearModel = LinearModel::from_file(model)?;
    println!("{:?}", model);

    Ok(())
}