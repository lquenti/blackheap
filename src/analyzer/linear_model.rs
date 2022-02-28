use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;

use std::fs::File;
use std::io::BufReader;

use serde::{Serialize, Deserialize};
use serde_json::json;

use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};


use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineStyle, LineJoin, PointMarker, PointStyle};
use plotlib::view::ContinuousView;


/// y=aX+b
#[derive(Serialize, Deserialize, Debug)]
pub struct LinearModel {
    pub a: f64,
    pub b: f64,
    // Just needed for plotting when creating the model
    max_access_size: Option<f64>
}

impl LinearModel {
    // TODO: A lot of double work with to_svg, rewrite me
    // TODO: From next minimum instead of maximum
    pub fn from_jsons_kdes(jsons: &Vec<BenchmarkJSON>, kdes: &Vec<BenchmarkKde>) -> Self {
        let max_access_size = Some(jsons[jsons.len()-1].access_size_in_bytes as f64);
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
            a, b, max_access_size
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
        let max_access_size = self.max_access_size.expect("max_access_size was None");
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

    pub fn to_json(&self) -> String {
        let ret = json!({
            "a": self.a,
            "b": self.b
        });
        ret.to_string()
    }

    pub fn from_file(model_path: &String) -> Result<Self, std::io::Error> {
        let file = File::open(model_path)?;
        let reader = BufReader::new(file);
        let res: LinearModel = serde_json::from_reader(reader)?;
        Ok(res)
    }
}
