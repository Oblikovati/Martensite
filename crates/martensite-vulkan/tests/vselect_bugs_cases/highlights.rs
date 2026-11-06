//! Highlight and selection colours after picking sub-shapes and objects.
use super::prelude::*;

/// Dumps of a selection made by point then xor, unhighlighted and with each picked part
/// detected; `dump_three_states(&mut run, "bug23649_1", [(232, 368), (165, 278)])?`.
fn dump_three_states(
    run: &mut SessionRun,
    images: [&'static str; 3],
    [first, second]: [(i32, i32); 2],
) -> Result<(), Box<dyn Error>> {
    run.session().select_point(first.0, first.1)?;
    run.session().select_polyline(&[second], Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump(images[0])?;
    run.session().move_cursor(first.0, first.1)?;
    run.dump(images[1])?;
    run.session().move_cursor(second.0, second.1)?;
    run.dump(images[2])
}

const OCC23649: &str = "0023649: Different behaviour of highlighting after shift selection.";

/// `bug23649_1`: highlighting of edges selected by point and xor.
pub(crate) fn shifted_sub_shape_highlights() -> Result<(), Box<dyn Error>> {
    banner(&[OCC23649, "It tests default functionality of highlighting"]);
    banner(&["during shift selection of sub-shapes (local context)."]);
    let b = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_object_selection_mode("b", 2, true)?;
    let images = [
        "bug23649_1_Selection.png",
        "bug23649_1_HighlightFirst.png",
        "bug23649_1_HighlightSecond.png",
    ];
    dump_three_states(&mut run, images, [(232, 368), (165, 278)])?;
    run.finish()
}

/// `bug23649_3`: highlighting of whole objects selected by point and xor.
pub(crate) fn shifted_object_highlights() -> Result<(), Box<dyn Error>> {
    banner(&[OCC23649, "It tests default functionality of highlighting"]);
    banner(&["during shift selection of shapes (neutral point)."]);
    let a = box_at([2., 0., 0.], [3., 2., 1.])?;
    let b = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    run.session().display("a", &a, &DisplayRequest::default())?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().fit()?;
    let images = [
        "bug23649_3_Selection.png",
        "bug23649_3_HighlightFirst.png",
        "bug23649_3_HighlightSecond.png",
    ];
    dump_three_states(&mut run, images, [(70, 230), (200, 358)])?;
    run.finish()
}

/// `bug26159`: a vertex is detected within the selection tolerance of the cursor.
pub(crate) fn vertex_tolerance_hit() -> Result<(), Box<dyn Error>> {
    banner(&[
        "================================================================",
        "CR26159",
        "Visualization - revise tolerance implementation for selection.",
        "================================================================",
        "",
    ]);
    let mut run = SessionRun::new()?;
    for (name, corner) in [("b1", 1.), ("b2", 3.), ("b3", 6.)] {
        let shape = box_at([corner; 3], [1., 1., 1.])?;
        run.session()
            .display(name, &shape, &DisplayRequest::default())?;
    }
    run.session().fit()?;
    run.session().set_object_selection_mode("b1", 1, true)?;
    run.session().move_cursor(58, 324)?;
    run.session().check_color(58, 324, [0., 1., 1.])?;
    run.dump("bug26159.png")?;
    run.finish()
}

/// `bug26721`: a selected face is highlighted with the selection colour, the rest unchanged.
pub(crate) fn face_highlight_color() -> Result<(), Box<dyn Error>> {
    banner(&["CR26721: Visualization - selection highlight of selected face is broken"]);
    let b = box_sized([1., 1., 1.])?;
    let mut run = SessionRun::new()?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().select_point(200, 200)?;
    run.session().set_selection_mode(4, true)?;
    run.session().select_point(200, 300)?;
    if run.session().pixel_rgba_name(292, 356)? != "YELLOW 1" {
        println!("ERROR: the box is drawn with the color that isn't equal to default!");
    }
    if run.session().pixel_rgba_name(204, 301)? != "GRAY80 1" {
        println!("ERROR: the face selected is not highlighted with selection color!");
    }
    run.dump("bug26721.png")?;
    run.finish()
}

/// `bug26641`: a zoom-persistent box is shown and picked in orthographic and perspective views.
pub(crate) fn persistent_object_picked() -> Result<(), Box<dyn Error>> {
    banner(&[
        "0026641: Visualization, Renderer - handle correctly transformation persistence within perspective projection",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let b0 = box_at([30., 0., 0.], [10., 10., 10.])?;
    let b1 = box_at([-30., -30., 30.], [10., 30., 20.])?;
    run.session()
        .display("b0", &b0, &DisplayRequest::default())?;
    run.session()
        .display("b1", &b1, &DisplayRequest::default())?;
    run.session().fit()?;
    run.dump("bug26641_none_pers_ortho.png")?;
    let persistent = DisplayRequest::zoom_persistent([-30., -30., 30.]);
    run.session().display("b1", &b1, &persistent)?;
    run.session().fit()?;
    run.dump("bug26641_zoom_pers_ortho.png")?;
    let perspective = martensite_tkservice::graphic3d::Graphic3dCameraProjection::Perspective;
    run.session().set_projection(perspective)?;
    run.session().fit()?;
    run.dump("bug26641_zoom_pers_persp.png")?;
    run.session().select_point(30, 70)?;
    if run.session().selected_count() != 1 {
        println!("ERROR: Transform persistence object is not selected!");
    }
    run.dump("bug26641.png")?;
    run.finish()
}
