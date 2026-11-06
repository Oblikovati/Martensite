//! Detection, highlighting and picked depth of circles as the circle command displays them.
use super::prelude::*;
use crate::viewer_objects::{circle_object, circle_through, point_object};
use martensite_tkernel::storage::format_general;

/// Display three named points and the circle through them;
/// `show_circle(&mut run, ["p1", "p2", "p3", "c"], points, true)?`.
fn show_circle(
    run: &mut SessionRun,
    names: [&str; 4],
    points: [[f64; 3]; 3],
    filled: bool,
) -> Result<(), Box<dyn Error>> {
    for (name, point) in names.iter().zip(points) {
        run.session().display_object(name, point_object(point)?)?;
    }
    let circle = circle_through(points)?;
    run.session()
        .display_object(names[3], circle_object(&circle, filled, None))?;
    Ok(())
}

/// The circle through (0, 0, 0), (50, 50, 0) and (100, 0, 0), fitted.
fn radial_circle(filled: bool) -> Result<SessionRun, Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    let points = [[0., 0., 0.], [50., 50., 0.], [100., 0., 0.]];
    show_circle(
        &mut run,
        ["radP1", "radP2", "radP3", "circle"],
        points,
        filled,
    )?;
    run.session().fit()?;
    Ok(run)
}

/// Print `message` unless pixel (x, y) has the named colour.
fn expect_named(
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

/// `bug22018`: of two overlapping filled circles, only the one under the cursor is detected.
pub(crate) fn overlapping_circles_detected() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC22018: Select3DSensitiveCircle::Matches(): DMin parameter left uninitialized",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    show_circle(
        &mut run,
        ["p1", "p2", "p3", "c"],
        [[-5., 0., 0.], [0., 5., 0.], [5., 0., 0.]],
        true,
    )?;
    show_circle(
        &mut run,
        ["p4", "p5", "p6", "c1"],
        [[0., 0., 0.], [5., 5., 0.], [10., 0., 0.]],
        true,
    )?;
    run.session().erase(&["p1", "p2", "p3", "p4", "p5", "p6"])?;
    run.session().fit()?;
    run.session()
        .set_standard_projection(martensite_tkv3d::v3d::V3dTypeOfOrientation::ZUP_TOP)?;
    let (x1, x2, y) = (150, 300, 200);
    hover_twice(&mut run, (0, 0))?;
    let unselected = "Error: unexpected color of not selected object";
    let selected = "Error: unexpected color of selected object";
    if run.session().pixel_color_name(x1, y)? != "IVORY"
        || run.session().pixel_color_name(x2, y)? != "IVORY"
    {
        println!("{unselected}");
    }
    hover_twice(&mut run, (x1, y))?;
    expect_named(&mut run, (x1, y), "WHITE", selected)?;
    expect_named(&mut run, (x2, y), "IVORY", unselected)?;
    hover_twice(&mut run, (x2, y))?;
    expect_named(&mut run, (x1, y), "IVORY", unselected)?;
    expect_named(&mut run, (x2, y), "WHITE", selected)?;
    run.dump("bug22018.png")?;
    run.finish()
}

/// The original moves the cursor to the same pixel twice.
fn hover_twice(run: &mut SessionRun, (x, y): (i32, i32)) -> Result<(), Box<dyn Error>> {
    run.session().move_cursor(x, y)?;
    run.session().move_cursor(x, y)?;
    Ok(())
}

/// `bug26217`: a rubber band over part of a filled circle selects it only with overlap allowed.
pub(crate) fn partial_rectangle_overlap() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR26217: Visualization, Select3DSensitiveCircle - fix compilation with CLang for iOS",
        "============",
        "",
    ]);
    let mut run = radial_circle(true)?;
    run.session()
        .select_pixels(&[(40, 120), (185, 320)], false)?;
    expect_named(
        &mut run,
        (120, 200),
        "DARKGOLDENROD",
        "ERROR: the circle is selected with no overlap mode activated",
    )?;
    run.session().select_point(0, 0)?;
    run.session()
        .select_pixels(&[(40, 120), (185, 320)], true)?;
    expect_named(
        &mut run,
        (120, 200),
        "GRAY73",
        "ERROR: the circle is not selected with overlap mode activated",
    )?;
    run.dump("bug26217.png")?;
    run.finish()
}

/// `bug27477`: the outline circle detected at its boundary reports a finite picked depth.
pub(crate) fn outline_circle_depth() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC27477: Visualization - Select3DSensitiveCircle always return infinite depth value in boundary mode",
        "============",
        "",
    ]);
    let mut run = radial_circle(false)?;
    run.session().move_cursor(177, 285)?;
    // The picked entity state prints each depth with `%g`; the first line is checked.
    let depth = run
        .session()
        .picked_depths()?
        .first()
        .map(|d| format_general(*d, 6));
    if depth.as_deref() == Some("17.4833") {
        println!("OK");
    } else {
        println!("ERROR: the depth value is incorrect: should be 17.4833, but is equal to:");
        println!("{}", depth.unwrap_or_else(|| "0".to_owned()));
    }
    run.dump("bug27477.png")?;
    run.finish()
}

/// The polyline of `bug31440`, shown with 2D persistence.
const BUG31440_LINE: [[f64; 3]; 6] = [
    [10., 9., 0.],
    [400., 9., 0.],
    [400., 209., 0.],
    [200., 109., 0.],
    [10., 209., 0.],
    [10., 9., 0.],
];

/// `bug31440`: a polygon only partly around a filled circle does not select it without
/// overlap detection, one around it does; then a polygon selects the box edges inside it and
/// the 2D polyline.
pub(crate) fn polygon_selection_without_overlap() -> Result<(), Box<dyn Error>> {
    use crate::case_view::viewer_session::{ViewParameters, corner_persistence};
    use crate::viewer_objects::polyline_object;
    use martensite_tkernel::quantity::{Color, NamedColor};
    use martensite_tkservice::aspect::AspectTypeOfTriedronPosition;
    use martensite_tkservice::graphic3d::Graphic3dTransModeFlags;
    banner(&[
        "============",
        "0031440: Visualization - Impossible to make common behaviour for multi-selection in viewer",
        "============",
        "",
    ]);
    let mut run = radial_circle(true)?;
    run.session()
        .select_pixels(&[(50, 300), (150, 300), (150, 100), (50, 100)], false)?;
    let message = "ERROR: the circle is selected with no overlap mode activated";
    expect_named(&mut run, (120, 200), "DARKGOLDENROD", message)?;
    run.session().select_pixels(&[(0, 0)], false)?;
    run.session()
        .select_pixels(&[(50, 300), (360, 300), (360, 100), (50, 100)], false)?;
    expect_named(
        &mut run,
        (120, 200),
        "GRAY73",
        "ERROR: the circle is not selected",
    )?;
    run.session().select_pixels(&[(0, 0)], false)?;
    run.session().clear()?;
    let b = box_at([0., 0., 0.], [1., 1., 1.])?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_object_selection_mode("b", 2, true)?;
    let scale = ViewParameters {
        scale: Some(500.),
        ..ViewParameters::default()
    };
    run.session().set_view_parameters(&scale)?;
    run.session()
        .display_object("line", polyline_object(&BUG31440_LINE)?)?;
    let pers = corner_persistence(
        Graphic3dTransModeFlags::TMF_2D,
        AspectTypeOfTriedronPosition::LEFT_LOWER,
        [0, 0],
    )?;
    run.session().redisplay_persistent("line", pers)?;
    run.session()
        .set_color("line", Color::named(NamedColor::RED))?;
    run.session().select_pixels(
        &[(10, 400), (400, 400), (400, 200), (200, 300), (10, 200)],
        false,
    )?;
    run.dump("bug31440.png")?;
    run.finish()
}
