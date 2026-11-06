use super::*;
use crate::case_view::viewer_session::{DisplayRequest, ViewParameters};
use crate::shape_checks::{MassCheck, MassKind, check_mass};
use crate::shape_report_checks::{ShapeCounts, check_shape_counts, check_shape_structure};
// Keep the authored decimal spellings for source provenance.
#[allow(clippy::excessive_precision)]
const CAMERA: ViewParameters = ViewParameters {
    scale: Some(19.469810863701095),
    proj: Some([
        0.99999862909317017,
        0.0012245246907696128,
        -0.0011169711360707879,
    ]),
    up: Some([
        0.00037844621692784131,
        0.48741284012794495,
        0.87317168712615967,
    ]),
    at: Some([290.970210143045, -0.0594423932820831, -1.29683163874688]),
    eye: None,
};
const HEADING: &[&str] = &[
    "TODO OCC11111 ALL: Error : The area of result shape is",
    "============",
    "OCC21415",
    "============",
    "",
];

fn positioned_view(shape: &Shape) -> Result<SessionRun, Box<dyn std::error::Error>> {
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", shape, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_display_mode(1)?;
    run.session().fit()?;
    run.session().set_view_parameters(&CAMERA)?;
    Ok(run)
}

pub(super) fn authored_checks(shape: &Shape) -> Result<(), Failure> {
    println!("TEMPORARY!!!!!!!!!!!!!!!!!");
    check_mass(shape, &MassCheck::near(MassKind::Area, 400000.))?;
    check_shape_structure(shape)?;
    println!("TEMPORARY!!!!!!!!!!!!!!!!!");
    check_shape_counts(
        shape,
        "result",
        &ShapeCounts::all([1, 2, 1, 1, 0, 0, 0, 0, 5]),
    )?;
    Ok(())
}

/// Preserve both fit calls, the authored camera and the required genuine area failure.
/// Example: `large_tolerance_presentation()?`.
pub(crate) fn large_tolerance_presentation() -> Outcome {
    banner(HEADING);
    let shape = read_dataset_shape("OCC21415-face_bad_meshed.brep")?;
    let mut run = positioned_view(&shape)?;
    authored_checks(&shape)?;
    run.dump("bug21415.png")?;
    run.finish()
}
