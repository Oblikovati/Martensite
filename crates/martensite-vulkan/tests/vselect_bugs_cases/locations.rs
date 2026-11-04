//! Highlight and selection structures follow objects moved through the context.
use super::prelude::*;

fn display_modes(display: i32, highlight: i32) -> DisplayRequest {
    DisplayRequest {
        display_mode: Some(display),
        highlight_mode: Some(highlight),
        ..DisplayRequest::default()
    }
}

/// `bug26960`: the dynamic highlight of a face moves with its object.
pub(crate) fn dynamic_highlight_moved() -> Result<(), Box<dyn Error>> {
    banner(&[
        "0026960: Visualization, Renderer - update transformation of dynamically highlighted presentation",
        "",
    ]);
    let b = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().display("b", &b, &display_modes(1, 1))?;
    run.session().fit()?;
    run.session().set_selection_mode(4, true)?;
    run.session().move_cursor(250, 250)?;
    if run.session().pixel_color_name(350, 140)? != "BLACK" {
        println!("ERROR: wrong initial location");
    }
    run.session().set_location("b", [0.5, 0., 0.])?;
    if run.session().pixel_color_name(350, 140)? != "CYAN2" {
        println!("ERROR: the transformation was not applied to highlight structure");
    }
    run.dump("bug26960.png")?;
    run.finish()
}

/// The box of OCC27986 shown in wireframe with a shaded highlight, zoomed out by half.
fn zoomed_wire_box(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    let b = box_sized([1., 2., 3.])?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().display("b", &b, &display_modes(0, 1))?;
    run.session().fit()?;
    run.session().zoom(0.5)?;
    Ok(())
}

/// Detect the box, move it, and check that the dynamic highlight moved too.
fn detect_and_move(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    run.session().move_cursor(250, 250)?;
    run.session().set_location("b", [2., 1., 0.])?;
    let first = run.session().pixel_color_name(220, 200)?;
    let second = run.session().pixel_color_name(350, 220)?;
    if first != "BLACK" || second != "CYAN2" {
        println!("Error: dynamic highlighting is not updated");
    }
    Ok(())
}

/// `bug27986_1`: the dynamic highlight follows SetLocation (global selection).
pub(crate) fn moved_global_highlight() -> Result<(), Box<dyn Error>> {
    banner(&["OCC27986"]);
    banner(&[
        "Visualization - AisInteractiveContext::SetLocation() does not update dynamic highlighting",
    ]);
    banner(&["Test global selection", ""]);
    let mut run = SessionRun::new()?;
    zoomed_wire_box(&mut run)?;
    detect_and_move(&mut run)?;
    run.dump("bug27986_1.png")?;
    run.finish()
}

/// `bug27986_2`: the dynamic and selection highlights follow SetLocation (local selection).
pub(crate) fn moved_local_highlight() -> Result<(), Box<dyn Error>> {
    banner(&[
        "0027986: Visualization - AisInteractiveContext::SetLocation() does not update dynamic highlighting",
    ]);
    banner(&["Test local selection", ""]);
    let mut run = SessionRun::new()?;
    zoomed_wire_box(&mut run)?;
    run.session().set_object_selection_mode("b", 4, true)?;
    run.session().select_point(150, 150)?;
    detect_and_move(&mut run)?;
    if run.session().pixel_color_name(150, 150)? != "BLACK" {
        println!("Error: sub-shape highlighting is not updated");
    }
    run.dump("bug27986_2.png")?;
    run.finish()
}

/// `bug28988`: an object moved and reset is detected again at its first place.
pub(crate) fn reset_location_detected() -> Result<(), Box<dyn Error>> {
    banner(&[
        "0028988: Visualization - AisInteractiveContext::SetLocation() does not reset location properly",
    ]);
    banner(&[""]);
    let b = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().display("b", &b, &display_modes(1, 1))?;
    run.session().fit()?;
    run.session().move_cursor(110, 110)?;
    if run.session().pixel_color_name(110, 110)? != "CYAN2" {
        println!("Error: shape is not selected!");
    }
    run.session().translate_location("b", [1., 0., 0.])?;
    run.session().move_cursor(110, 110)?;
    run.session().reset_location("b")?;
    run.session().move_cursor(110, 110)?;
    if run.session().pixel_color_name(110, 110)? != "CYAN2" {
        println!("Error: reset shape is not selected!");
    }
    run.dump("bug28988.png")?;
    run.finish()
}

/// `bug30672`: an object removed from the context, moved and displayed again is drawn and
/// detected at its new location.
pub(crate) fn removed_object_relocated() -> Result<(), Box<dyn Error>> {
    banner(&[
        "0030672: Visualization - AisInteractiveContext::SetLocation() sets invalid selection BVH flag to removed object",
    ]);
    let b = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().display("b", &b, &display_modes(1, 1))?;
    run.session().fit()?;
    run.session().remove_from_context("b")?;
    run.session().set_location_translation("b", [2., 0., 0.])?;
    let shaded = DisplayRequest {
        display_mode: Some(1),
        ..DisplayRequest::default()
    };
    run.session().display("b", &b, &shaded)?;
    run.session().move_cursor(200, 200)?;
    if run.session().pixel_color_name(150, 150)? != "BLACK" {
        println!("Error: wrong prs location");
    }
    if run.session().pixel_color_name(300, 300)? != "DARKGOLDENROD" {
        println!("Error: wrong sel location");
    }
    run.session().move_cursor(300, 300)?;
    if run.session().pixel_color_name(300, 300)? != "CYAN2" {
        println!("Error: wrong highlighting");
    }
    run.dump("bug30672.png")?;
    run.finish()
}
