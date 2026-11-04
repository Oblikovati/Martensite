//! Highlighting, fitting, redisplaying and erasing the current selection.
use super::prelude::*;

const OCC23649: &str = "0023649: Different behaviour of highlighting after shift selection";

/// Select by point then xor, enable highlighting of selected objects, and dump each picked
/// part detected; `highlight_selected_states(&mut run, images, [(232, 368), (165, 278)])?`.
fn highlight_selected_states(
    run: &mut SessionRun,
    images: [&'static str; 2],
    [first, second]: [(i32, i32); 2],
) -> Result<(), Box<dyn Error>> {
    run.session().select_point(first.0, first.1)?;
    run.session().select_polyline(&[second], Xor)?;
    run.session().move_cursor(0, 0)?;
    run.session().set_highlight_selected(true);
    run.session().move_cursor(first.0, first.1)?;
    run.dump(images[0])?;
    run.session().move_cursor(second.0, second.1)?;
    run.dump(images[1])
}

/// `bug23649_2`: selected edges stay highlightable with 'highlight selected' on.
pub(crate) fn highlight_selected_sub_shapes() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        OCC23649,
        "It tests highlighting of selected sub-shapes.",
    ]);
    banner(&[
        "This mode is option of interactive context.",
        "============",
        "",
    ]);
    let b = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_object_selection_mode("b", 2, true)?;
    let images = [
        "bug23649_2_HighlightFirst.png",
        "bug23649_2_HighlightSecond.png",
    ];
    highlight_selected_states(&mut run, images, [(232, 368), (165, 278)])?;
    run.finish()
}

/// `bug23649_4`: selected objects stay highlightable with 'highlight selected' on.
pub(crate) fn highlight_selected_objects() -> Result<(), Box<dyn Error>> {
    banner(&["============", OCC23649]);
    banner(&["It tests highlighting of selected shapes (neutral point)."]);
    banner(&[
        "This mode is option of interactive context.",
        "============",
        "",
    ]);
    let a = box_at([2., 0., 0.], [3., 2., 1.])?;
    let b = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    run.session().display("a", &a, &DisplayRequest::default())?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().fit()?;
    let images = [
        "bug23649_4_HighlightFirst.png",
        "bug23649_4_HighlightSecond.png",
    ];
    highlight_selected_states(&mut run, images, [(70, 230), (200, 358)])?;
    run.finish()
}

/// `bug26209`: the view fits the selected sub-shapes.
pub(crate) fn fit_to_selection() -> Result<(), Box<dyn Error>> {
    banner(&["============"]);
    banner(&["CR26209: Visualization - provide a method to fit view to the specific bounding box"]);
    banner(&["============", ""]);
    let b = box_sized([1., 1., 1.])?;
    let mut run = SessionRun::new()?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_object_selection_mode("b", 2, true)?;
    run.session().set_object_selection_mode("b", 1, true)?;
    run.session().select_point(330, 334)?;
    run.session().fit_selected()?;
    run.session().check_color(330, 334, [0., 0., 0.])?;
    run.session().check_color(330, 131, [0.8, 0.8, 0.8])?;
    run.session().select_point(0, 0)?;
    run.session().fit()?;
    run.session().select_point(29, 104)?;
    run.session().select_polyline(&[(204, 2)], Xor)?;
    run.session().fit_selected()?;
    run.session().check_color(29, 104, [0., 0., 0.])?;
    run.session().check_color(2, 317, [0.8, 0.8, 0.8])?;
    run.dump("bug26209.png")?;
    run.finish()
}

/// `bug22016`: a selected object displayed again keeps the selection colour.
pub(crate) fn redisplayed_selection_color() -> Result<(), Box<dyn Error>> {
    banner(&["========"]);
    banner(&["OCC22016: Incorrect selection color after displaying a selected object"]);
    banner(&["========", ""]);
    let b1 = box_at([0., 0., 0.], [1., 2., 3.])?;
    let b2 = box_at([3., 0., 0.], [2., 3., 1.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session()
        .display("b1", &b1, &DisplayRequest::default())?;
    run.session()
        .display("b2", &b2, &DisplayRequest::default())?;
    run.session().set_line_width("b1", 4.)?;
    run.session().set_line_width("b2", 4.)?;
    run.session().fit()?;
    run.session()
        .select_rectangle((0, 0), (500, 500), Replace)?;
    run.session()
        .display("b2", &b2, &DisplayRequest::default())?;
    let first = run.session().pixel_color_name(104, 32)?;
    let second = run.session().pixel_color_name(255, 350)?;
    if first != "GRAY80" || second != "GRAY80" {
        println!(
            "ERROR: OCC22016 is reproduced. Incorrect selection color after displaying a selected object."
        );
    }
    run.dump("bug22016.png")?;
    run.finish()
}

/// `bug26726`: erasing the selected objects leaves the view empty.
pub(crate) fn erase_selected_objects() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC26726 erase selected objects",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let shaded = DisplayRequest {
        display_mode: Some(1),
        ..DisplayRequest::default()
    };
    let corners = [[5., 0., 0.], [0., 5., 0.], [0., 0., 5.], [5., 5., 5.]];
    for (name, corner) in ["b0", "b1", "b2", "b3"].into_iter().zip(corners) {
        let shape = box_at(corner, [1., 1., 1.])?;
        run.session().display(name, &shape, &shaded)?;
    }
    run.session().fit()?;
    run.dump("bug26726_0.png")?;
    run.session()
        .select_rectangle((0, 0), (500, 500), Replace)?;
    run.session().erase_selected()?;
    run.dump("bug26726_1.png")?;
    let probes = [(205, 355), (205, 190), (60, 100), (350, 100)];
    let mut erased = true;
    for (x, y) in probes {
        erased &= run.session().pixel_color_name(x, y)? == "BLACK";
    }
    if !erased {
        println!(
            "Error: OCC26726 is reproduced. AisInteractiveContext::EraseSelected is incorrect."
        );
    }
    run.finish()
}
