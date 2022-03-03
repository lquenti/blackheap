use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;
use std::error::Error;

use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;
use crate::subprograms::use_model::CsvLine;
use crate::benchmark_wrapper::BenchmarkType;

use serde::{Serialize, Deserialize};
use serde_json;

use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineStyle, LineJoin, PointMarker, PointStyle};
use plotlib::view::ContinuousView;

#[derive(Debug, Serialize, Deserialize)]
pub struct LinearModels(Vec<LinearModelJSON>);

impl LinearModels {
    // TODO: Definitely need anyhow or Either or alike
    pub fn from_file(file_path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let file: File = File::open(file_path)?;
        let reader = BufReader::new(file);
        let x: Result<Self, serde_json::Error> = serde_json::from_reader(reader);
        match x {
            Ok(v) => Ok(v),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn find_lowest_upper_bound(&self, line: &CsvLine) -> Option<&LinearModelJSON> {
        let mut res = None;
        for lm in self.0.iter() {
            // Apples and oranges
            if lm.is_read_op != (line.io_type == 'r') {
                continue;
            }

            let approximated_time = lm.model.evaluate(line.bytes);

            // we are looking for an upper bound. Thus if it is lower, we can instantly reject it.
            if approximated_time < line.sec {
                continue
            }

            // do we have a upper bound already?
            res = match res {
                // if not, this is the best until now
                None => Some(lm),
                // if so, lets choose the tighter bound
                Some(lm2) => Some(if lm2.model.evaluate(line.bytes) < approximated_time { lm2 } else { lm }),
            }
        }
        res
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinearModelJSON {
    pub benchmark_type: BenchmarkType,
    pub is_read_op: bool,
    pub model: LinearModel,
}

// TODO Probably refactor me away
/// y=aX+b
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinearModel {
    pub a: f64,
    pub b: f64,
}

impl LinearModel {
    // TODO: A lot of double work with to_svg, rewrite me
    // TODO: From next minimum instead of maximum
    pub fn from_jsons_kdes(jsons: &Vec<BenchmarkJSON>, kdes: &Vec<BenchmarkKde>) -> Self {
        let (xs, ys) = Self::get_xs_ys(jsons, kdes);
        let data = vec![("X", xs), ("Y", ys)];
        let formula = "Y ~ X";
        let data = RegressionDataBuilder::new().build_from(data).unwrap();
        let model = FormulaRegressionBuilder::new()
        .data(&data)
        .formula(formula)
        .fit().unwrap();

        let parameters = model.parameters;
        let a = parameters.regressor_values[0];
        let b = parameters.intercept_value;
        Self {
            a, b
        }
    }

    fn get_xs_ys(jsons: &Vec<BenchmarkJSON>, kdes: &Vec<BenchmarkKde>) -> (Vec<f64>, Vec<f64>) {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for i in 0..jsons.len() {
            xs.push(jsons[i].access_size_in_bytes as f64);
            ys.push(kdes[i].get_global_maximum().0);
        }
        (xs, ys)
    }

    // TODO refactor me as well
    pub fn to_svg(&self, jsons: &Vec<BenchmarkJSON>, kdes: &Vec<BenchmarkKde>) -> String {
        // they are expected to be ordered TODO validate
        let max_access_size = jsons[jsons.len()-1].access_size_in_bytes as f64;
        let (xs, ys) = Self::get_xs_ys(jsons, kdes);
        let xs_ys: Vec<(f64, f64)> = xs.iter().cloned().zip(ys.iter().cloned()).collect();
        let pts = Plot::new(xs_ys).point_style(
            PointStyle::new()
            .colour("#ff0000")
            .marker(PointMarker::Cross)
        );
        let line = Plot::new(vec![(0.0f64, self.b), (max_access_size, max_access_size* self.a)])
            .line_style(
                LineStyle::new()
                .colour("#0000ff")
                .linejoin(LineJoin::Round)
            );
        let v = ContinuousView::new()
            .add(line)
            .add(pts)
            .x_label("Access Sizes in Bytes")
            .y_label("Expected Size in sec");
        Page::single(&v).to_svg().unwrap().to_string()
    }

    pub fn evaluate(&self, bytes: u64) -> f64 {
        self.a * (bytes as f64) + self.b
    }
}
