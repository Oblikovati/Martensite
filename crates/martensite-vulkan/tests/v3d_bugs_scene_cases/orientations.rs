//! Standard view orientations, the projection size, a flipped output, rotated face locations
//! and free boundary colours.
use super::prelude::*;
use crate::boolean_typed_explode::all_distinct;
use crate::case_view::viewer_session::FreeBoundary;
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeTorus;
use martensite_tkservice::graphic3d::Graphic3dCameraProjection;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::v3d::V3dTypeOfOrientation as Orientation;

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

/// The checks of `bug23253` after each orientation: two pixels and their linear colours.
type OrientationChecks = (Option<Orientation>, [((i32, i32), [f64; 3]); 2]);

/// The orientation steps of `bug23253`; `None` is the axonometric view.
fn bug23253_steps() -> [OrientationChecks; 7] {
    let (gold, bright) = ([0.91, 0.64, 0.11], [1., 1., 0.94]);
    [
        (None, [((120, 260), gold), ((120, 260), gold)]),
        (
            Some(Orientation::ZUP_TOP),
            [((125, 135), [1., 1., 0.92]), ((220, 200), bright)],
        ),
        (
            Some(Orientation::ZUP_BACK),
            [((120, 230), [0.92, 0.66, 0.11]), ((190, 170), bright)],
        ),
        (
            Some(Orientation::ZUP_FRONT),
            [((120, 230), [0.93, 0.66, 0.11]), ((220, 170), bright)],
        ),
        (
            Some(Orientation::ZUP_RIGHT),
            [((120, 230), [0.92, 0.66, 0.11]), ((240, 170), bright)],
        ),
        (
            Some(Orientation::ZUP_LEFT),
            [((120, 230), [0.93, 0.66, 0.11]), ((170, 170), bright)],
        ),
        (
            Some(Orientation::ZUP_BOTTOM),
            [((90, 300), [0.81, 0.58, 0.09]), ((220, 250), bright)],
        ),
    ]
}

/// `bug23253`: a shaded box and torus seen along each standard orientation.
pub(crate) fn standard_orientations() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC23253: Additional viewer 3d draw-commands vleft, vfront, ...",
    ]);
    banner(&["============", ""]);
    let b = box_sized([10., 20., 30.])?;
    let t = BRepPrimAPIMakeTorus::new(50., 10.)?.shape()?.clone();
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    show(&mut run, "b", &b)?;
    show(&mut run, "t", &t)?;
    run.session().fit()?;
    for (orientation, checks) in bug23253_steps() {
        match orientation {
            None => run.session().set_axonometric()?,
            Some(o) => run.session().set_standard_projection(o)?,
        }
        for ((x, y), color) in checks {
            run.session().check_color(x, y, color)?;
        }
    }
    run.dump("bug23253.png")?;
    run.finish()
}

/// `bug24714`: a projection size of 22 puts the detected box edge at the view top.
pub(crate) fn projection_size_set() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR24714: Test V3dView::SetSize method",
        "============",
        "",
    ]);
    let mut run = SessionRun::with_view_size(400, 200)?;
    run.session().clear()?;
    show(&mut run, "b", &box_sized([10., 10., 10.])?)?;
    run.session()
        .set_standard_projection(Orientation::ZUP_FRONT)?;
    run.session().fit()?;
    run.session().update_view(|view| view.set_size(22.))?;
    run.session().set_line_width("b", 5.)?;
    run.session().move_cursor(200, 100)?;
    let message = "Error : the view projection size is incorrect!";
    expect_named(&mut run, (200, 9), "CYAN", message)?;
    run.dump("bug24714.png")?;
    run.finish()
}

/// `bug32073`: a shaded cone from the front, then the same view with its output flipped.
pub(crate) fn flipped_output() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "0032073: Visualization - implement interface to change myToFlipOutput of GlView",
    ]);
    banner(&["========", ""]);
    let c = cone(10., 0., 10.)?;
    let mut run = SessionRun::new()?;
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().display("c", &c, &shaded)?;
    run.session()
        .set_projection(Graphic3dCameraProjection::Orthographic)?;
    run.session()
        .set_standard_projection(Orientation::ZUP_FRONT)?;
    run.session().fit()?;
    expect_named(
        &mut run,
        (100, 250),
        "DARKGOLDENROD",
        "Error: view is flipped",
    )?;
    run.dump("bug32073.png")?;
    run.session().set_flip_output(true)?;
    expect_named(
        &mut run,
        (100, 250),
        "DARKGOLDENROD",
        "Error: view is not flipped",
    )?;
    run.dump("bug32073_flipped.png")?;
    run.finish()
}

/// `bug31673`: rotations applied to a location compose after the current location.
pub(crate) fn location_rotations_composed() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "0031673: Test Commands, ViewerCommands - vlocation -rotate is applied in opposite order",
    ]);
    banner(&["============", ""]);
    let faces = all_distinct(&box_at([-1., -1., -1.], [2., 2., 2.])?, ShapeEnum::Face);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    for (numbers, mode) in [(&[1, 3, 4, 6][..], 0), (&[2, 5][..], 1)] {
        let request = DisplayRequest::default().with_display_mode(mode);
        for &number in numbers {
            run.session()
                .display(&format!("b_{number}"), &faces[number - 1], &request)?;
        }
    }
    run.session().fit()?;
    run.session().zoom(0.4)?;
    run.session().reset_location("b_2")?;
    run.session().reset_location("b_5")?;
    let y = [0., 1., 0.];
    run.session()
        .rotate_location("b_2", [1., -1., -1.], y, 90.)?;
    run.session()
        .rotate_location("b_2", [-1., -1., -1.], y, 90.)?;
    run.session()
        .rotate_location("b_5", [-1., -1., -1.], y, 90.)?;
    if run.session().pixel_color_name(170, 380)? == "BLACK" {
        println!("Error: wrong rotation");
    }
    run.dump("bug31673.png")?;
    run.finish()
}

/// A linear RGB colour of the aspects command (`0 0 1`).
fn rgb(r: f64, g: f64, b: f64) -> Result<Color, Box<dyn Error>> {
    Ok(Color::new(r, g, b, ColorSpace::Rgb)?)
}

/// `bug25773`: free boundary colour, visibility and width set on a face reach its drawing.
pub(crate) fn free_boundary_properties() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR25773",
        "Check that properties of object can be applied to it (set to its drawer)",
    ]);
    banner(&["============", ""]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let blue = FreeBoundary {
        color: Some(rgb(0., 0., 1.)?),
        ..FreeBoundary::default()
    };
    run.session().set_default_free_boundary(&blue)?;
    let face = all_distinct(&cone(0., 5., 10.)?, ShapeEnum::Face)[0].clone();
    show(&mut run, "c_1", &face)?;
    run.session().fit()?;
    let (x, y) = (80, 103);
    run.session().check_color(x, y, [0., 0., 1.])?;
    let hidden = FreeBoundary {
        draw: Some(false),
        ..FreeBoundary::default()
    };
    run.session().set_free_boundary("c_1", &hidden)?;
    run.session().check_color(x, y, [0., 0., 0.])?;
    let red = FreeBoundary {
        draw: Some(true),
        width: Some(2.),
        color: Some(rgb(1., 0., 0.)?),
    };
    run.session().set_free_boundary("c_1", &red)?;
    run.session().check_color(x, y, [1., 0., 0.])?;
    run.dump("bug25773.png")?;
    run.finish()
}
