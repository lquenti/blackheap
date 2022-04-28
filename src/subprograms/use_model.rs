use crate::analyzer::Analysis;
use crate::benchmark_wrapper::BenchmarkType;

use serde::Deserialize;

use anyhow::Result;

// TODO move me somewhere else
#[derive(Debug, Deserialize)]
pub struct CsvLine {
    pub filename: String,
    pub io_type: char,
    pub bytes: u64,
    pub sec: f64,
}

impl CsvLine {
    fn from_file(file: &str) -> Result<Vec<CsvLine>, std::io::Error> {
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
struct UseModelOutput {
    bytes_sec: Vec<(f64, f64)>,
    number_of_classified: Vec<((BenchmarkType, bool), u64)>,
}

pub fn use_model(model: &str, file: &str) -> Result<()> {
    // TODO: validate

    // get measurements
    let measurements: Vec<CsvLine> = CsvLine::from_file(file)?;

    // load Analyzed
    let analyzed = Analysis::load_from_file(model)?;

    for m in &measurements {
        let oa = Analysis::find_lowest_upper_bound(&analyzed, m);
        println!(
            "{}: {} bytes in {} took less than {} ({} {})",
            if m.io_type == 'r' {"read"} else {"write"},
            m.bytes,
            m.sec,
            match &oa {
                None => String::from("<NONE>"),
                Some(a) => a.linear_model.evaluate(m.bytes).to_string(),
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


    /*
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
    */

    Ok(())
}
