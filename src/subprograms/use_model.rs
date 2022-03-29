use std::path::PathBuf;

use crate::analyzer::linear_model::LinearModels;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CsvLine {
    pub filename: String,
    pub io_type: char,
    pub bytes: u64,
    pub sec: f64,
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
    for m in &measurements {
        println!("{:?}", m);
    }
    println!("----------");

    // load LinearModel
    let models = LinearModels::from_file(&PathBuf::from(model)).unwrap();
    for m in models.iter() {
        println!("{:?}", m);
    }
    println!("----------");

    // debug
    for m in &measurements {
        let olm = models.find_lowest_upper_bound(m);
        println!(
            "{}: {} bytes in {} took less than {} ({} {})",
            if m.io_type == 'r' { "read" } else { "write" },
            m.bytes,
            m.sec,
            match &olm {
                None => String::from("<NONE>"),
                Some(lm) => lm.model.evaluate(m.bytes).to_string(),
            },
            match &olm {
                None => String::from(""),
                Some(lm) => format!("{}", lm.benchmark_type),
            },
            match &olm {
                None => String::from(""),
                Some(lm) => String::from(if lm.is_read_op { "read" } else { "write" }),
            },
        );
        println!("----------");
    }

    Ok(())
}
