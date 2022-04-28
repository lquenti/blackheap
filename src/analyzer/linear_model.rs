use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;

use serde::{Deserialize, Serialize};

use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct LinearModels(Vec<LinearModelJSON>);

impl LinearModels {
    // TODO: Definitely need anyhow or Either or alike
    /*
    pub fn from_file(file_path: &Path) -> Result<Self, Box<dyn Error>> {
        let file: File = File::open(file_path)?;
        let reader = BufReader::new(file);
        let x: Result<Self, serde_json::Error> = serde_json::from_reader(reader);
        match x {
            Ok(v) => Ok(v),
            Err(e) => Err(Box::new(e)),
        }
    }
    */

    pub fn find_lowest_upper_bound(&self, line: &CsvLine) -> Option<&LinearModelJSON> {
        let mut res = None;
        for lm in self.0.iter() {
            // Apples and oranges
            if lm.is_read_op != (line.io_type == 'r') {
                continue;
            }
            println!("{:?}", lm);

            let approximated_time = lm.model.evaluate(line.bytes);
            println!("{:?} -> {}", lm, approximated_time);

            // we are looking for an upper bound. Thus if it is lower, we can instantly reject it.
            if approximated_time < line.sec {
                continue;
            }

            // do we have a upper bound already?
            res = match res {
                // if not, this is the best until now
                None => Some(lm),
                // if so, lets choose the tighter bound
                Some(lm2) => Some(if lm2.model.evaluate(line.bytes) < approximated_time {
                    lm2
                } else {
                    lm
                }),
            }
        }
        res
    }

    // TODO debug
    pub fn iter(&self) -> std::slice::Iter<LinearModelJSON> {
        self.0.iter()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinearModelJSON {
    pub benchmark_type: BenchmarkType,
    pub is_read_op: bool,
    pub model: LinearModel,
}
*/

// TODO Probably refactor me away
/// y=aX+b
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinearModel {
    pub a: f64,
    pub b: f64,
}

impl LinearModel {
    pub fn from_jsons_kdes(jsons: &[BenchmarkJSON], kdes: &[BenchmarkKde]) -> Self {
        let (xs, ys) = Self::get_xs_ys(jsons, kdes);
        let data = vec![("X", xs), ("Y", ys)];
        let formula = "Y ~ X";
        let data = RegressionDataBuilder::new().build_from(data).unwrap();
        let model = FormulaRegressionBuilder::new()
            .data(&data)
            .formula(formula)
            .fit()
            .unwrap();

        let parameters = model.parameters;
        let a = parameters.regressor_values[0];
        let b = parameters.intercept_value;
        Self { a, b }
    }

    pub fn get_xs_ys(jsons: &[BenchmarkJSON], kdes: &[BenchmarkKde]) -> (Vec<f64>, Vec<f64>) {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for i in 0..jsons.len() {
            xs.push(jsons[i].access_size_in_bytes as f64);
            ys.push(kdes[i].global_maximum.0);
        }
        (xs, ys)
    }
}
