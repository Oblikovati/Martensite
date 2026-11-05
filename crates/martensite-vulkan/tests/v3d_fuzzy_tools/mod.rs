#[cfg(test)]
mod cpu;
mod geometry;
use crate::case_view::viewer_session::DisplayRequest;
use crate::session_run::{SessionRun, banner};
use crate::shape_checks::{MassCheck, MassKind, check_mass};
use crate::shape_report_checks::{ShapeCounts, check_shape_counts, check_shape_structure};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::topods::Shape;
type BoxError = Box<dyn std::error::Error>;
const COUNTS: [usize; 9] = [32, 51, 20, 20, 1, 1, 0, 1, 126];
const FUZZY: f64 = 0.00001;

fn actual_fuzzy_five_tool_fuse() -> Result<Shape, BoxError> {
    let shapes = geometry::source_six_inputs()?;
    let mut filler = crate::common_operation::configured_intersections(&shapes[0], &shapes[1]);
    filler.options_mut().set_fuzzy_value(FUZZY);
    for tool in &shapes[2..] {
        filler.add_argument(tool);
    }
    filler.perform();
    crate::common_operation::report_result_alerts(filler.options());
    if filler.options().has_errors() {
        return Err("bfillds errors=true; expected completed fuzzy intersections".into());
    }
    complete_fuzzy_fuse(&shapes, &mut filler)
}

fn complete_fuzzy_fuse(
    shapes: &[Shape; 6],
    filler: &mut martensite_tkbo::bop_algo::BOPAlgoPaveFiller,
) -> Result<Shape, BoxError> {
    let mut operation =
        crate::common_operation::configured_prepared(&shapes[0], &shapes[1], Operation::Fuse);
    operation.options_mut().set_fuzzy_value(FUZZY);
    for tool in &shapes[2..] {
        operation.add_tool(tool);
    }
    operation.perform_with_filler(filler);
    completed_fuzzy_result(operation)
}

fn completed_fuzzy_result(
    operation: martensite_tkbo::bop_algo::BOPAlgoBOP,
) -> Result<Shape, BoxError> {
    crate::common_operation::report_result_alerts(operation.options());
    if operation.options().has_errors() || operation.shape().is_null() {
        return Err(format!(
            "bbop Fuse errors={},null={}; expected completed non-null result",
            operation.options().has_errors(),
            operation.shape().is_null()
        )
        .into());
    }
    Ok(operation.shape().clone())
}

fn case_geometry_checks(shape: &Shape) -> Result<(), BoxError> {
    check_mass(shape, &MassCheck::near(MassKind::Area, 1893.17))?;
    check_shape_structure(shape)?;
    check_shape_counts(shape, "result", &authored_fuzzy_expectation())?;
    Ok(())
}

/// Preserve all sector transforms, explicit fuzzy setting and authored result checks.
/// Example: `fuzzy_multi_tool_view()?`.
pub(crate) fn fuzzy_multi_tool_view() -> Result<(), BoxError> {
    banner(&["============", "OCC25880", "============", ""]);
    let result = actual_fuzzy_five_tool_fuse()?;
    case_geometry_checks(&result)?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().set_display_mode(1)?;
    run.session().fit()?;
    run.dump("bug25880.png")?;
    run.finish()
}

/// Retain the authored located-count flag and multi-tool result report subject.
/// Example: `check_shape_counts(&shape, "result", &authored_fuzzy_expectation())?`.
fn authored_fuzzy_expectation() -> ShapeCounts {
    ShapeCounts {
        per_location: true,
        subject: "fuzzy booleans with multiple tools",
        ..ShapeCounts::all(COUNTS)
    }
}
