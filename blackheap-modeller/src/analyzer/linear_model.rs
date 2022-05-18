use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;

use serde::{Deserialize, Serialize};

use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

// TODO: Rename file
// TODO: Do serialization/deserialization via function only
// TODO: Hide access to a/b
// TODO: Rename a/b
pub trait PredictionModel {
    fn from_jsons_kdes(jsons: &[BenchmarkJSON], kdes: &[BenchmarkKde]) -> Self;
    fn evaluate(&self, bytes: u64) -> f64;
    // helper
    fn get_xs_ys(jsons: &[BenchmarkJSON], kdes: &[BenchmarkKde]) -> (Vec<f64>, Vec<f64>) {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for i in 0..jsons.len() {
            xs.push(jsons[i].access_size_in_bytes as f64);
            ys.push(kdes[i].global_maximum.0);
        }
        (xs, ys)
    }
}

/*
/// y=aX+b
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinearModel {
    pub a: f64,
    pub b: f64,
}

impl PredictionModel for LinearModel {
    fn from_jsons_kdes(jsons: &[BenchmarkJSON], kdes: &[BenchmarkKde]) -> Self {
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
    fn evaluate(&self, bytes: u64) -> f64 {
        self.a * (bytes as f64) + self.b
    }
}
*/

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Interval {
    lower: Option<f64>,
    upper: Option<f64>,
}

impl Interval {
    // Creates an unrestricted interval, i.e. |R
    pub fn new() -> Self {
        Self {lower: None, upper: None}
    }

    pub fn new_left_closed(minimum: f64) -> Self{
        Self {lower: Some(minimum), upper: None}
    }

    pub fn new_right_closed(maximum: f64) -> Self{
        Self {lower: None, upper: Some(maximum)}
    }

    pub fn new_closed(minimum: f64, maximum: f64) -> Self{
        Self {lower: Some(minimum), upper: Some(maximum)}
    }

    pub fn contains(&self, val: f64) -> Self {
        if let Some(minimum) = val {
            if val
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Constant {
    valid_interval: Interval,
    const_value: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Linear {
    valid_interval: Interval,
    slope: f64,
    y_intercept: f64,
}

// TODO: Check if sth like dependent typing allows that the valid interval can't overlap
// TODO: or any other compile time constraint, who knows
// Otherwise, just check it on runtime
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ConstLinear {
    constant: Constant,
    linear: Linear
}

impl PredictionModel for Linear {
    fn from_jsons_kdes(jsons: &[BenchmarkJSON], kdes: &[BenchmarkKde]) -> Self {
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
        let slope = parameters.regressor_values[0];
        let y_intercept = parameters.intercept_value;
    }
}
