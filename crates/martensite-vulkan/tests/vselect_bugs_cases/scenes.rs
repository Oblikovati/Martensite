//! Removal of displayed objects, picking of an infinite face, fitting to a moved selection and
//! the selection highlight drawn in the objects' own mode.
use super::prelude::*;
use crate::dataset_shape::read_dataset_shape;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::geom::{GeomCylindricalSurface, GeomSurface};
use martensite_tkmath::gp::{Axis3, Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

/// `bug26146`: a compound of point sets is shown, then removed without a crash.
pub(crate) fn point_set_compound_removed() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "Test for issue #26146",
        "Select3DISensitivePointSet: Crash when clearing selections (only with TBB)",
        "========",
        "",
    ]);
    let compound = read_dataset_shape("bug26146_crash_compound.brep")?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("crash_compound", &compound, &DisplayRequest::default())?;
    run.session().fit()?;
    run.dump("bug26146_1.png")?;
    run.session().remove(&["crash_compound"])?;
    run.dump("bug26146_2.png")?;
    run.finish()
}

/// The face of the infinite cylinder of radius 10 along +Z through (0, 0, 1).
fn infinite_cylinder_face() -> Result<Shape, Box<dyn Error>> {
    let (z, x) = (Direction3::new(0., 0., 1.)?, Direction3::new(1., 0., 0.)?);
    let frame = Axis3::new(Point3::new(0., 0., 1.), z, x)?;
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomCylindricalSurface::new(frame, 10.)?).cast();
    let maker = BRepBuilderAPIMakeFace::from_surface(&surface, precision::confusion())?;
    Ok(maker.face()?.into_shape())
}

/// `bug129_1`: an infinite cylindrical face is detected, selected and deselected.
pub(crate) fn infinite_cylinder_selected() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "OCC129 - infinite cylinder is not selectable in 3D Viewer",
        "========",
        "",
    ]);
    let f = infinite_cylinder_face()?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .display("f", &f, &DisplayRequest::default().with_display_mode(0))?;
    run.session().move_cursor(207, 184)?;
    run.session().check_color(207, 184, [0., 1., 1.])?;
    run.session().select_point(207, 184)?;
    run.session().check_color(207, 184, [0.8, 0.8, 0.8])?;
    run.session().select_point(222, 175)?;
    run.session().check_color(207, 184, [1., 1., 0.])?;
    run.dump("bug129_1.png")?;
    run.finish()
}

/// `bug1164_1`: fitting to a selected face of a moved box centres the face.
pub(crate) fn moved_face_fitted() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "Verify vfit -selected works correctly for a primitive TopoDS_Shape with transformation",
        "========",
    ]);
    let b = box_sized([100., 200., 300.])?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("b", &b, &DisplayRequest::default().with_display_mode(1))?;
    run.session().set_location("b", [500., 0., 0.])?;
    run.session().fit()?;
    run.session().zoom(0.5)?;
    run.session().set_object_selection_mode("b", 4, true)?;
    run.session().select_point(250, 125)?;
    run.session().fit_selected()?;
    if run.session().pixel_color_name(250, 250)? == "BLACK" {
        println!("Error: face should be in the middle");
    }
    run.finish_without_image()
}

/// `bug32482`: removing a detected object also clears it from the picked owners.
pub(crate) fn removed_object_not_picked() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=============",
        "0032482: Visualization - Object owner isn't removed from picked owner when object is removed",
        "=============",
    ]);
    banner(&[""]);
    let shaded = DisplayRequest::default().with_display_mode(1);
    let mut run = SessionRun::new()?;
    run.session()
        .display("b1", &box_sized([1., 2., 3.])?, &shaded)?;
    run.session()
        .display("b2", &box_at([-0.5; 3], [3., 2., 1.])?, &shaded)?;
    run.session().fit()?;
    run.session().move_cursor(200, 200)?;
    println!("{:?}", run.session().picked_entity_types()?);
    run.session().remove(&["b1"])?;
    println!("{:?}", run.session().picked_entity_types()?);
    run.dump("bug32482.png")?;
    run.finish()
}

const OCC22048: &str = "0022048: Visualization, AisInteractiveContext - single object selection should always clear multiple selection";

/// Colour names expected at the two boxes, with the message printed otherwise.
type ColorChecks = [((i32, i32), &'static str, &'static str); 2];

const BOTH_SELECTED: ColorChecks = [
    ((50, 300), "GRAY73", "Error: b1 should be selected."),
    ((300, 200), "GRAY73", "Error: b2 should be selected."),
];

const SECOND_SELECTED: ColorChecks = [
    (
        (50, 300),
        "DARKGOLDENROD",
        "Error: b1 should not be selected.",
    ),
    ((300, 200), "GRAY73", "Error: b2 should be selected."),
];

/// Two shaded boxes in an axonometric view, the selection highlighted in the objects' mode.
fn two_shaded_boxes() -> Result<SessionRun, Box<dyn Error>> {
    let shaded = DisplayRequest::default().with_display_mode(1);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    let b1 = box_at([0.; 3], [10.; 3])?;
    let b2 = box_at([20.; 3], [30.; 3])?;
    run.session().display("b1", &b1, &shaded)?;
    run.session().display("b2", &b2, &shaded)?;
    run.session().fit()?;
    run.session().set_selection_display_mode(-1)?;
    Ok(run)
}

/// `bug22048`: a single pick replaces a multiple selection made by a xor rectangle.
pub(crate) fn single_pick_clears_multiple() -> Result<(), Box<dyn Error>> {
    banner(&["=========", OCC22048, "========="]);
    let mut run = two_shaded_boxes()?;
    run.session().select_rectangle((0, 0), (400, 400), Xor)?;
    expect_colors(&mut run, &BOTH_SELECTED)?;
    run.session().select_point(200, 200)?;
    expect_colors(&mut run, &SECOND_SELECTED)?;
    run.dump("bug22048.png")?;
    run.finish()
}

/// Print each check's message unless its pixel has the expected colour name.
fn expect_colors(run: &mut SessionRun, checks: &ColorChecks) -> Result<(), Box<dyn Error>> {
    for ((x, y), name, message) in checks {
        if run.session().pixel_color_name(*x, *y)? != *name {
            println!("{message}");
        }
    }
    Ok(())
}
