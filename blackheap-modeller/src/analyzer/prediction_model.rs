use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;

use serde::{Deserialize, Serialize};

use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

#[derive(Debug, Serialize, Deserialize)]
pub enum Models {
    Linear(Linear),
    ConstantLinear(ConstantLinear),
}

impl Models {
    pub fn new_linear(jsons: &[BenchmarkJSON], kdes: &[BenchmarkKde], xss: Interval) -> Self {
        Models::Linear(Linear::from_jsons_kdes_interval(jsons, kdes, xss))
    }
    pub fn new_constant_linear(
        jsons: &[BenchmarkJSON],
        kdes: &[BenchmarkKde],
        xss: Interval,
    ) -> Self {
        Models::ConstantLinear(ConstantLinear::from_jsons_kdes_interval(jsons, kdes, xss))
    }
}

// TODO document what xs_ys are
fn get_xs_ys_interval(
    jsons: &[BenchmarkJSON],
    kdes: &[BenchmarkKde],
    xss: Interval,
) -> (Vec<f64>, Vec<f64>) {
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    for i in 0..jsons.len() {
        // if codomain in valid interval, it is relevant for our analysis
        if xss.contains(jsons[i].access_size_in_bytes) {
            xs.push(jsons[i].access_size_in_bytes as f64);
            ys.push(kdes[i].global_maximum.0);
        }
    }
    (xs, ys)
}
fn find_max_xs_ys(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let (mut max_xs, mut max_ys): (f64, f64) = (0.0f64, 0.0f64);
    for i in 0..xs.len() {
        let (curr_xs, curr_ys): (f64, f64) = (xs[i], ys[i]);
        if curr_ys > max_ys {
            max_xs = curr_xs;
            max_ys = curr_ys;
        }
    }
    (max_xs, max_ys)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Interval {
    pub lower: Option<u64>,
    pub upper: Option<u64>,
}

impl Interval {
    // Creates an unrestricted interval, i.e. |R
    pub fn new() -> Self {
        Self {
            lower: None,
            upper: None,
        }
    }

    pub fn new_left_closed(minimum: u64) -> Self {
        Self {
            lower: Some(minimum),
            upper: None,
        }
    }

    pub fn new_right_closed(maximum: u64) -> Self {
        Self {
            lower: None,
            upper: Some(maximum),
        }
    }

    pub fn new_closed(minimum: u64, maximum: u64) -> Self {
        Self {
            lower: Some(minimum),
            upper: Some(maximum),
        }
    }

    pub fn contains(&self, val: u64) -> bool {
        if let Some(minimum) = self.lower {
            // its lower bounded
            if val < minimum {
                return false;
            }
        }
        if let Some(maximum) = self.upper {
            // its upper bounded
            if val > maximum {
                return false;
            }
        }
        true
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Constant {
    pub valid_interval: Interval,
    pub const_value: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Linear {
    pub valid_interval: Interval,
    pub slope: f64,
    pub y_intercept: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConstantLinear {
    pub constant: Constant,
    pub linear: Linear,
}

// TODO Explain why duplicated
impl Constant {
    pub fn from_jsons_kdes_interval(
        jsons: &[BenchmarkJSON],
        kdes: &[BenchmarkKde],
        xss: Interval,
    ) -> Self {
        let (xs, ys): (Vec<f64>, Vec<f64>) = get_xs_ys_interval(jsons, kdes, xss);
        let (_max_xs, max_ys): (f64, f64) = find_max_xs_ys(&xs, &ys);
        Self {
            const_value: max_ys,
            valid_interval: xss,
        }
    }
}

impl Linear {
    pub fn from_jsons_kdes_interval(
        jsons: &[BenchmarkJSON],
        kdes: &[BenchmarkKde],
        xss: Interval,
    ) -> Self {
        let (xs, ys): (Vec<f64>, Vec<f64>) = get_xs_ys_interval(jsons, kdes, xss);
        let data: Vec<(&str, Vec<f64>)> = vec![("X", xs), ("Y", ys)];
        let formula: &str = "Y ~ X";
        let data: linregress::RegressionData =
            RegressionDataBuilder::new().build_from(data).unwrap();
        let model: linregress::RegressionModel = FormulaRegressionBuilder::new()
            .data(&data)
            .formula(formula)
            .fit()
            .unwrap();

        let parameters: linregress::RegressionParameters = model.parameters;
        let slope: f64 = parameters.regressor_values[0];
        let y_intercept: f64 = parameters.intercept_value;

        let valid_interval: Interval = xss;

        Self {
            slope,
            y_intercept,
            valid_interval,
        }
    }
}

impl ConstantLinear {
    pub fn from_jsons_kdes_interval(
        jsons: &[BenchmarkJSON],
        kdes: &[BenchmarkKde],
        xss: Interval,
    ) -> Self {
        if !xss.contains(4096) {
            panic!("splitting didn't work. Unrecoverable. Quitting...");
        }
        // if xss is unbounded, let it be unbounded as well
        let lower_interval: Interval = match xss.lower {
            None => Interval::new_right_closed(4096),
            Some(lower_bound) => Interval::new_closed(lower_bound, 4096),
        };
        let upper_interval: Interval = match xss.upper {
            None => Interval::new_left_closed(4096),
            Some(upper_bound) => Interval::new_closed(4096, upper_bound),
        };

        let constant = Constant::from_jsons_kdes_interval(jsons, kdes, lower_interval);
        let linear = Linear::from_jsons_kdes_interval(jsons, kdes, upper_interval);

        Self { constant, linear }
    }
}
