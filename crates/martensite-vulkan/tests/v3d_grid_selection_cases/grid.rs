//! Real world/pixel conversion on the authored camera and privileged planes.
use super::*;
use crate::case_view::viewer_session::{GridLayout, GridRequest, ViewParameters};
use martensite_tkservice::{aspect::AspectGridDrawMode, graphic3d::Graphic3dCameraProjection};
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

fn rectangular(session: &ViewerSession, origin: (f64, f64), step: f64) -> Result<(), Failure> {
    let layout = GridLayout {
        origin: Some(origin),
        step: Some((step, step)),
        ..Default::default()
    };
    session.show_grid(&GridRequest::rectangular(layout).drawn_as(AspectGridDrawMode::Lines))
}

/// Apply the authored camera, depth and grid; `prepare_shifted(&session)?`.
pub(super) fn prepare_shifted(session: &ViewerSession) -> Result<(), Failure> {
    let parameters = ViewParameters {
        scale: Some(60.630934227306405),
        eye: Some([15., -5., 15.]),
        at: Some([5., 5., 5.]),
        proj: Some([0.577, -0.577, 0.577]),
        up: Some([-0.408, 0.408, 0.816]),
    };
    session.set_view_parameters(&parameters)?;
    session.update_view(|view| view.camera().set_z_range(-300., 300.))?;
    rectangular(session, (-10., 10.), 1.)
}

/// Change the real privileged plane; `plane(&session, [20., 20., 30.])?`.
pub(super) fn plane(session: &ViewerSession, origin: [f64; 3]) -> Result<(), Failure> {
    session.set_privileged_plane(origin, [0., 0., 1.], [1., 0., 0.])
}

/// Compute the authored world-to-grid point; `world_grid(&session, 30.)?`.
pub(super) fn world_grid(session: &ViewerSession, z: f64) -> Result<(f64, f64, f64), Failure> {
    session.view().view_base().convert_point_to_grid(5., 5., z)
}

const SHIFTED_BANNER: [&str; 4] = [
    "============",
    "CR25672: V3dView::ConvertToGrid doesn't work for grid if the grid plane's origin is not identical to camera origin.",
    "============",
    "",
];

fn check_world_grid(session: &ViewerSession, z: f64, expected: [f64; 3]) -> Result<(), Failure> {
    let actual = world_grid(session, z)?;
    let label = format!("vconvert 5.0 5.0 {z:.1} grid");
    report_grid(&label, actual, expected, true);
    Ok(())
}

/// Run every translated-plane numeric check and dump; `shifted_grid()?`.
pub(crate) fn shifted_grid() -> Outcome {
    banner(&SHIFTED_BANNER);
    let mut run = SessionRun::new()?;
    prepare_shifted(run.session())?;
    check_world_grid(run.session(), 0., [5., 5., 0.])?;
    plane(run.session(), [20., 20., 30.])?;
    check_world_grid(run.session(), 30., [5., 5., 30.])?;
    run.session()
        .set_projection(Graphic3dCameraProjection::Perspective)?;
    plane(run.session(), [10., 5., 10.])?;
    check_world_grid(run.session(), 10., [5., 5., 10.])?;
    run.dump("bug25672.png")?;
    let diagnostic = super::grid_regular_control::capture(run.session());
    run.retain_observation(diagnostic);
    run.finish()
}

/// Use the command's pixel-to-world-to-grid branch; `pixel_grid_point(&session)?`.
pub(super) fn pixel_grid_point(session: &ViewerSession) -> Result<(f64, f64, f64), Failure> {
    let view = session.view().view_base();
    let (x, y, z) = view.convert_to_world(200, 200)?;
    view.convert_point_to_grid(x, y, z)
}

/// Activate the authored five-unit grid; `prepare_pixel(&session)?`.
pub(super) fn prepare_pixel(session: &ViewerSession) -> Result<(), Failure> {
    rectangular(session, (0., 0.), 5.)
}

/// Change the real projection before converting; `pixel_orientation(&session, false)?`.
pub(super) fn pixel_orientation(
    session: &ViewerSession,
    top: bool,
) -> Result<(f64, f64, f64), Failure> {
    if top {
        session.set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    } else {
        session.set_axonometric()?;
    }
    pixel_grid_point(session)
}

/// Lift the real privileged plane by ten; `lift_pixel_grid(&session)?`.
pub(super) fn lift_pixel_grid(session: &ViewerSession) -> Result<(), Failure> {
    plane(session, [0., 0., 10.])
}

fn check_pixel_stage(session: &ViewerSession, lifted: bool) -> Result<(), Failure> {
    let (expected, suffix) = if lifted {
        lift_pixel_grid(session)?;
        ([[15., -15., 10.], [0., -5., 10.]], "lifted")
    } else {
        ([[5., -5., 0.], [0., -5., 0.]], "origin")
    };
    for ((top, name), wanted) in [(false, "axo"), (true, "top")].into_iter().zip(expected) {
        let actual = pixel_orientation(session, top)?;
        let label = format!("vconvert {name} - grid {suffix}");
        report_grid(&label, actual, wanted, false);
    }
    Ok(())
}

/// Run all four projections at 400 square pixels and dump; `pixel_grid()?`.
pub(crate) fn pixel_grid() -> Outcome {
    banner(&[
        "============",
        "CR26435: Visualization - V3dView::ConvertToGrid returns wrong coordinates for non-orthogonal projection",
        "============",
        "",
    ]);
    let mut run = SessionRun::with_view_size(400, 400)?;
    prepare_pixel(run.session())?;
    check_pixel_stage(run.session(), false)?;
    check_pixel_stage(run.session(), true)?;
    run.dump("bug26435.png")?;
    run.finish()
}
