//! Full argument-check diagnostics retain actual named faulty compounds without a command-host database.
mod groups;
mod output;
use crate::case_argument_response::{
    VALID_ARGUMENT_RESPONSE, configured_default_argument_analyzer,
};
use martensite_tkbo::bop_algo::{BOPAlgoArgumentAnalyzer, Operation};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};

/// Own the response and topology-command-equivalent bindings through the caller's END; `report.output`.
pub(crate) struct RetainedFullArgumentReport {
    /// Exact interpreter response, including ordered binding announcements.
    pub output: String,
    /// Actual faulty compounds in source binding order, without deriving identity from rank.
    pub bindings: Vec<(String, Shape)>,
    /// Retain the geometry copy used by the pair analyzer through the caller's END.
    pub copied_second: Option<Shape>,
}

/// Execute `bopargcheck first #F`, or `first second -F #F`; `full_argument_report(&shape, None)?`.
pub(crate) fn full_argument_report(
    first: &Shape,
    second: Option<&Shape>,
) -> Result<RetainedFullArgumentReport, Failure> {
    refuse_null(first, "first")?;
    let mut analyzer = configured_default_argument_analyzer(first);
    let copied_second = configure_second(&mut analyzer, second)?;
    analyzer.perform();
    assembled_report(&analyzer, copied_second)
}

fn assembled_report(
    analyzer: &BOPAlgoArgumentAnalyzer,
    copied_second: Option<Shape>,
) -> Result<RetainedFullArgumentReport, Failure> {
    let mut report = RetainedFullArgumentReport {
        output: String::new(),
        bindings: Vec::new(),
        copied_second,
    };
    if !analyzer.has_faulty() {
        report.output.push_str(VALID_ARGUMENT_RESPONSE);
        return Ok(report);
    }
    groups::collect(analyzer, &mut report)?;
    Ok(report)
}

fn refuse_null(shape: &Shape, argument: &str) -> Result<(), Failure> {
    if shape.is_null() {
        return Err(Failure::with_message(
            "Standard_NullObject",
            &format!("bopargcheck {argument}=null; expected non-null shape"),
        ));
    }
    Ok(())
}

fn configure_second(
    analyzer: &mut BOPAlgoArgumentAnalyzer,
    second: Option<&Shape>,
) -> Result<Option<Shape>, Failure> {
    let Some(second) = second else {
        return Ok(None);
    };
    refuse_null(second, "second")?;
    let mut copier = BRepBuilderAPICopy::with_shape(second, true, false)?;
    let copied = copier.shape()?.clone();
    analyzer.set_shape2(&copied);
    *analyzer.operation_type() = Operation::Fuse;
    *analyzer.tangent_mode() = true;
    *analyzer.merge_vertex_mode() = true;
    *analyzer.merge_edge_mode() = true;
    Ok(Some(copied))
}
