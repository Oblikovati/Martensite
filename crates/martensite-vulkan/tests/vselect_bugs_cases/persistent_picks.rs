//! Dynamic highlighting of a zoom-persistent object in front of a restored face.
use super::prelude::*;
use crate::case_view::viewer_session::ViewParameters;
use crate::dataset_shape::read_dataset_shape;

/// Print `message` unless pixel (x, y) has the named colour; `expect_pixel(&run, 1, 2, "CYAN", "ERROR")`.
fn expect_pixel(
    run: &mut SessionRun,
    (x, y): (i32, i32),
    name: &str,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    if run.session().pixel_color_name(x, y)? != name {
        println!("{message}");
    }
    Ok(())
}

/// The first camera: the box in front of the face, seen from below.
fn first_camera() -> [ViewParameters; 3] {
    [
        ViewParameters {
            scale: Some(588.7),
            proj: Some([0.69, -0.64, -0.38]),
            ..Default::default()
        },
        ViewParameters {
            up: Some([-0.16, -0.32, 0.93]),
            at: Some([-0.47, -0.042, 0.4]),
            ..Default::default()
        },
        ViewParameters {
            eye: Some([0.7, -1.14, -0.17]),
            ..Default::default()
        },
    ]
}

/// The precision camera: the box edge next to the face edge.
fn precision_camera() -> [ViewParameters; 3] {
    [
        ViewParameters {
            scale: Some(689.79),
            proj: Some([0.78, 0.63, -0.00067]),
            ..Default::default()
        },
        ViewParameters {
            up: Some([-0.13, 0.16, -0.98]),
            at: Some([-0.36, -0.016, 0.31]),
            ..Default::default()
        },
        ViewParameters {
            eye: Some([0.96, 1.053, 0.31]),
            ..Default::default()
        },
    ]
}

/// Apply the three camera steps in order; `apply(&mut run, first_camera())?`.
fn apply(run: &mut SessionRun, lines: [ViewParameters; 3]) -> Result<(), Box<dyn Error>> {
    for line in &lines {
        run.session().set_view_parameters(line)?;
    }
    Ok(())
}

/// `bug26719_1`: the zoom-persistent box, not the face behind it, is detected, also at the
/// box's border.
pub(crate) fn persistent_box_detected_over_face() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "0026719: Visualization - cannot pick zoom persistent object",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    let f = read_dataset_shape("face1.brep")?;
    run.session().display("f", &f, &DisplayRequest::default())?;
    let b1 = box_sized([25., 25., 25.])?;
    let persistent = DisplayRequest::zoom_persistent([0., 0., 0.]);
    run.session().display("b1", &b1, &persistent)?;
    apply(&mut run, first_camera())?;
    run.session().move_cursor(280, 290)?;
    expect_pixel(
        &mut run,
        (297, 297),
        "CYAN",
        "ERROR: zoom persistent box is not highlighted dynamically!",
    )?;
    expect_pixel(
        &mut run,
        (372, 210),
        "GOLDENROD3",
        "ERROR: the shape behind zoom persistent object was highlighted instead!",
    )?;
    run.dump("bug26719_1_1.png")?;
    precision_checks(&mut run)?;
    run.finish()
}

/// The second and third dumps: the box border, then the face just beside it, detected.
fn precision_checks(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    run.session().move_cursor(0, 0)?;
    apply(run, precision_camera())?;
    run.session().move_cursor(264, 135)?;
    expect_pixel(
        run,
        (275, 142),
        "CYAN",
        "ERROR: zoom persistent box is not highlighted dynamically in precision test!",
    )?;
    expect_pixel(
        run,
        (243, 123),
        "BURLYWOOD2",
        "ERROR: the shape behind zoom persistent object was highlighted instead in precision test!",
    )?;
    run.dump("bug26719_1_2.png")?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(259, 135)?;
    expect_pixel(
        run,
        (275, 142),
        "DARKGOLDENROD",
        "ERROR: zoom persistent box is highlighted instead in precision test!",
    )?;
    expect_pixel(
        run,
        (243, 123),
        "CYAN",
        "ERROR: the shape behind zoom persistent object was not highlighted dynamically in precision test!",
    )?;
    run.dump("bug26719_1_3.png")
}
