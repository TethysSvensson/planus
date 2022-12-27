use askama::Template;

use crate::{
    analysis::run_analysis, backend_translation::run_backend, dot::DotBackend, rust::RustBackend,
};

pub mod analysis;
pub mod backend;
pub mod backend_translation;
pub mod dot;
pub mod rust;
pub mod templates;

#[derive(thiserror::Error, Debug)]
pub enum CodegenError {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("codegen error: {0}")]
    Other(String),
}

pub fn generate_rust(
    declarations: &planus_types::intermediate::Declarations,
) -> Result<String, CodegenError> {
    let default_analysis = run_analysis(declarations, &mut rust::analysis::DefaultAnalysis);
    let eq_analysis = run_analysis(declarations, &mut rust::analysis::EqAnalysis);
    let infallible_analysis = run_analysis(
        declarations,
        &mut rust::analysis::InfallibleConversionAnalysis,
    );
    let output = run_backend(
        &mut RustBackend {
            default_analysis,
            eq_analysis,
            infallible_analysis,
        },
        declarations,
    );
    let res = templates::rust::Namespace(&output).render().unwrap();
    let res = rust::format_string(&res)?;
    Ok(res)
}

pub fn generate_dot(declarations: &planus_types::intermediate::Declarations) -> String {
    let output = run_backend(&mut DotBackend { color_seed: 0 }, declarations);
    let res = templates::dot::Namespace(&output).render().unwrap();
    res
}