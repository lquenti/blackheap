
use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;
use crate::analyzer::linear_model::LinearModel;

use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "result.stpl")]
pub struct ResultTemplate<'a> {
    pub benchmark_name: String,
    pub jsons_kdes: Vec<(&'a BenchmarkJSON, &'a BenchmarkKde)>,
    pub linear_model: LinearModel,
    pub linear_model_svg: String
}