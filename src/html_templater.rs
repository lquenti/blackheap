
use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;
use crate::analyzer::linear_model::LinearModel;

pub use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "result.stpl")]
pub struct ResultTemplate<'a> {
    benchmark_name: String,
    jsons_kdes: Vec<(&'a BenchmarkJSON, &'a BenchmarkKde)>,
    linear_model: LinearModel,
    linear_model_svg: String
}