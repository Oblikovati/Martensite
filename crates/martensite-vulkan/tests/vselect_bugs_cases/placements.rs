//! Detection after the view is turned or placed and after selected objects are moved.
use super::prelude::*;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

/// `bug25103`: walls detected one after the other in a turned front view.
pub(crate) fn turned_walls_detected() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR25103: Visualization - Regression in interactive detection",
    ]);
    banner(&["============", ""]);
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    let walls = [
        ("wall1", [1., 8., 8.], [-10., 0., 0.]),
        ("wall2", [1., 8., 8.], [5., 0., 0.]),
        ("wall3", [16., 8., 1.], [-10., 0., -1.]),
    ];
    for (name, size, _) in walls {
        run.session()
            .display(name, &box_sized(size)?, &DisplayRequest::default())?;
    }
    for (name, _, location) in walls {
        run.session().set_location(name, location)?;
    }
    use Graphic3dNameOfMaterial::{Pewter, Stone};
    for (name, material) in [("wall1", Stone), ("wall2", Stone), ("wall3", Pewter)] {
        run.session().set_material(name, material)?;
    }
    run.session()
        .set_color("wall1", Color::named(NamedColor::RED))?;
    run.session()
        .set_color("wall2", Color::named(NamedColor::GREEN))?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_FRONT)?;
    run.session()
        .update_view(|view| view.turn(0., -0.3, 0., true))?;
    run.session().fit()?;
    run.dump("bug25103_1.png")?;
    run.session().move_cursor(394, 205)?;
    run.dump("bug25103_2.png")?;
    run.session().move_cursor(264, 205)?;
    run.dump("bug25103_3.png")?;
    run.finish()
}

/// `bug25335`: an edge of a moved box is picked after each move and displayed as its own
/// shape at the place it was selected.
pub(crate) fn moved_box_edges_picked() -> Result<(), Box<dyn Error>> {
    banner(&[
        "================",
        "OCC25335: Visualization - method ::SelectedShape() in AisLocalContext and AisInteractiveContext overrides shape transformation",
    ]);
    banner(&["================", ""]);
    let mut run = SessionRun::new()?;
    run.session()
        .display("b", &box_sized([1., 1., 1.])?, &DisplayRequest::default())?;
    run.session().set_selection_mode(2, true)?;
    run.session().fit()?;
    run.session().select_point(299, 352)?;
    run.session().pick_selected("PickedShape_0_")?;
    run.session().set_location("b", [0., 1., 0.])?;
    run.session().z_fit()?;
    run.session().select_point(233, 188)?;
    run.session().pick_selected("PickedShape_1_")?;
    run.session().set_location("b", [-2., 2., 0.])?;
    run.session().fit()?;
    run.dump("bug25335.png")?;
    run.finish()
}

/// `bug25507`: placing a picked point at the view centre keeps the moved box detectable.
pub(crate) fn placed_view_detected() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR25507: Visualization - the method V3dView::Place() is incorrect",
    ]);
    banner(&["============", ""]);
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    run.session()
        .display("b1", &box_sized([2., 1., 1.])?, &DisplayRequest::default())?;
    run.session()
        .display("b2", &box_sized([1., 1., 1.])?, &DisplayRequest::default())?;
    run.session().set_location("b2", [5., 0., 0.])?;
    run.session().fit()?;
    run.session().select_point(80, 140)?;
    run.session()
        .update_view(|view| view.place(80, 140, view.scale()))?;
    run.session().move_cursor(200, 180)?;
    run.session().check_color(200, 180, [0.78, 0.55, 0.09])?;
    run.dump("bug25507.png")?;
    run.finish()
}

/// `bug30824`: a rectangle around the sphere selects the parent only, not its in-place
/// children.
pub(crate) fn parent_selected_without_children() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=============",
        "0030824: Visualization, PrsMgrPresentableObject - A new flag to disable automatic selection of children",
    ]);
    banner(&["============="]);
    let mut run = SessionRun::new()?;
    crate::child_scenes::boxes_beside_sphere(&mut run)?;
    run.session().select_pixels(&[(0, 0), (200, 200)], false)?;
    run.dump("bug30824.png")?;
    run.finish()
}
