use crate::analyzer::json_reader::BenchmarkJSON;
use crate::analyzer::kde::BenchmarkKde;
use crate::analyzer::linear_model::LinearModel;
use crate::analyzer::Analysis;

use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "single_model.stpl")]
pub struct SingleModelTemplate<'a> {
    pub benchmark_name: String,
    pub op: String,
    pub jsons: &'a Vec<BenchmarkJSON>,
    pub kdes: &'a Vec<BenchmarkKde>,
    pub linear_model: &'a LinearModel,
    pub linear_model_svg: String,
}

impl<'a> SingleModelTemplate<'a> {
    pub fn from_analysis(a: &'a Analysis) -> Self {
        SingleModelTemplate {
            benchmark_name: a.benchmark.benchmark_type.to_string(),
            op: String::from(if a.benchmark.is_read_op {
                "read"
            } else {
                "write"
            }),
            jsons: &a.jsons,
            kdes: &a.kdes,
            linear_model: &a.linear_model,
            linear_model_svg: a.linear_model.to_svg(&a.jsons, &a.kdes),
        }
    }
    pub fn to_html_string(self) -> String {
        self.render_once().unwrap()
    }
}

#[derive(TemplateOnce)]
#[template(path = "model_summary.stpl")]
pub struct ModelSummaryTemplate<'a> {
    pub analyzed: &'a Vec<Analysis>,
    pub svg_all: String,
}

impl<'a> ModelSummaryTemplate<'a> {
    pub fn to_html_string(self) -> String {
        self.render_once().unwrap()
    }
}
