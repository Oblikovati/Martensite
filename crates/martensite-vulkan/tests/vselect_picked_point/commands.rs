//! Preserve the source's three mode changes, live picks and exact formatted-point comparisons.
use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
use crate::picked_point_observations::{
    FilesystemPointObservationWriter, record_point_observation,
};
use crate::picked_point_text::source_point_diagnostics;
use crate::primitive_shapes::box_sized;
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::standard::Failure;
use martensite_tkv3d::select_mgr::SelectMgrViewerSelectorResultMembers as _;
use std::error::Error;
use std::path::PathBuf;

/// Execute the complete original against its real viewer; `complete_picked_point()?`.
pub(crate) fn complete_picked_point() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "0029938: Visualization - SelectMgrViewerSelector::PickedPoint() should return point lying on an object",
        "============",
        "",
    ]);
    let shape = box_sized([1., 2., 3.])?;
    // Core-profile GL would keep raster lines one pixel wide.
    // Preserve the AIS highlight's requested width; select the driver contract instead.
    let mut run = SessionRun::with_line_width_policy(
        martensite_vulkan::RenderLineWidthPolicy::SinglePixelV1,
    )?;
    prepare_picked_point_scene(run.session(), &shape)?;
    check_picked_point_modes(run.session())?;
    run.dump("bug29938.png")?;
    run.finish()
}

fn prepare_picked_point_scene(
    session: &mut ViewerSession,
    shape: &martensite_tkbrep::topods::Shape,
) -> Result<(), Failure> {
    // A fresh isolated context starts clear; Vulkan replaces the core GL backend.
    session.set_axonometric()?;
    let display = DisplayRequest::default().with_display_mode(1).mutable();
    session.display("b", shape, &display)?;
    session.fit()?;
    session.rotate(0.2, 0., 0.)?;
    session.set_selection_mode(0, false)
}

fn check_picked_point_modes(session: &mut ViewerSession) -> Result<(), Box<dyn Error>> {
    session.set_selection_mode(1, true)?;
    check_live_point(session, 1, (337, 45), "1 2 3")?;
    session.set_selection_mode(1, false)?;
    session.set_selection_mode(2, true)?;
    check_live_point(session, 2, (295, 317), "1 2 0")?;
    session.set_selection_mode(2, false)?;
    session.set_selection_mode(4, true)?;
    check_live_point(session, 4, (284, 324), "1 1.8383 0")
}

fn check_live_point(
    session: &mut ViewerSession,
    mode: i32,
    pixel: (i32, i32),
    expected: &str,
) -> Result<(), Box<dyn Error>> {
    session.move_cursor(pixel.0, pixel.1)?;
    let points = ranked_state_points(session)?;
    preserve_picked_state(mode, pixel, &points)?;
    for diagnostic in source_point_diagnostics(&points, expected) {
        println!("{diagnostic}");
    }
    Ok(())
}

fn preserve_picked_state(mode: i32, pixel: (i32, i32), points: &[[f64; 3]]) -> std::io::Result<()> {
    let folder = std::env::var_os("MARTENSITE_HATCH_CAPTURE_DIR").map(PathBuf::from);
    record_point_observation(
        folder.as_deref(),
        mode,
        pixel,
        points,
        &FilesystemPointObservationWriter,
    )
}

fn ranked_state_points(session: &ViewerSession) -> Result<Vec<[f64; 3]>, Failure> {
    let selector = session.context().main_selector();
    (1..=selector.nb_picked())
        .map(|rank| {
            // ViewerCommands prints PickedData.Point, not a fabricated shape location.
            let point = selector.picked_data(rank)?.point;
            Ok([point.x(), point.y(), point.z()])
        })
        .collect()
}
