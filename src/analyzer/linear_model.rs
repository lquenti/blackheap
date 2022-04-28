use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;

use serde::{Deserialize, Serialize};

use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

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
