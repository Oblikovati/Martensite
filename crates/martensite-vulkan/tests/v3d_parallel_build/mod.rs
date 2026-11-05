#[cfg(test)]
mod cpu;
mod operation;
use crate::case_view::viewer_session::DisplayRequest;
use crate::dataset_shape::read_dataset_shape;
use crate::session_run::{SessionRun, banner};
use crate::shape_checks::{MassCheck, MassKind, check_mass};
use crate::shape_report_checks::{ShapeCounts, check_shape_counts, check_shape_structure};
use martensite_tkbrep::topods::{Shape, ShapeIterator};
type BoxError = Box<dyn std::error::Error>;

fn source_direct_members() -> Result<Vec<Shape>, BoxError> {
    let shape = read_dataset_shape("bug26701_crash_tbb.brep")?;
    Ok(ShapeIterator::new(&shape, true, true).collect())
}

fn case_parallel_result_checks(shape: &Shape) -> Result<(), BoxError> {
    check_mass(shape, &MassCheck::near(MassKind::Area, 8244.36))?;
    check_shape_structure(shape)?;
    check_shape_counts(shape, "result", &authored_topology_expectation())?;
    Ok(())
}

/// Preserve direct explode order, explicit parallelism and every authored result check.
/// Example: `parallel_general_build_view()?`.
pub(crate) fn parallel_general_build_view() -> Result<(), BoxError> {
    banner(&["============", "OCC26701", "============", ""]);
    let result = operation::parallel_general_build(&source_direct_members()?)?;
    case_parallel_result_checks(&result)?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().set_display_mode(1)?;
    run.session().fit()?;
    run.dump("bug26701.png")?;
    run.finish()
}

/// Preserve the source `checknbshapes -t -m` count mode and literal subject.
/// Example: `check_shape_counts(&shape, "result", &authored_topology_expectation())?`.
fn authored_topology_expectation() -> ShapeCounts {
    ShapeCounts {
        per_location: true,
        subject: "result obtained by Common operator",
        ..ShapeCounts::all([19252, 19264, 4824, 1620, 1600, 0, 0, 1, 46561])
    }
}
