//! Detection and selection presentations of erased, cleared and recomputed objects.
use super::prelude::*;

fn shaded() -> DisplayRequest {
    DisplayRequest {
        display_mode: Some(1),
        ..DisplayRequest::default()
    }
}

/// `bug26596`: an object erased twice is no longer detected.
pub(crate) fn twice_erased_not_detected() -> Result<(), Box<dyn Error>> {
    banner(&["============"]);
    banner(&["CR26596: Visualization - hidden twice times structure should not be selectabl"]);
    banner(&["============", ""]);
    let b = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().set_display_mode(1)?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().erase(&["b"])?;
    run.session().display("b", &b, &DisplayRequest::default())?;
    run.session().erase(&["b"])?;
    run.session().move_cursor(250, 250)?;
    if run.session().pixel_rgba(165, 263)? == [0., 1., 1., 1.] {
        println!(
            "ERROR: Zero selection mode was not deactivated at second AisInteractiveContext::Erase call"
        );
    }
    run.dump("bug26596.png")?;
    run.finish()
}

/// Detect a shaded box, dump it, remove it by `hide`, and dump the empty view.
fn detected_then_hidden(
    title: &str,
    images: [&'static str; 2],
    hide: fn(&mut SessionRun) -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    banner(&["========", title, "========", ""]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let b = box_sized([1., 2., 3.])?;
    run.session().display("b", &b, &shaded())?;
    run.session().fit()?;
    run.session().move_cursor(250, 250)?;
    run.dump(images[0])?;
    hide(&mut run)?;
    run.dump(images[1])?;
    run.finish()
}

/// `bug27621_1`: the detection highlight disappears with the erased object.
pub(crate) fn detected_then_erased() -> Result<(), Box<dyn Error>> {
    detected_then_hidden(
        "CR27621_1: Highlighting with color remains visible when shape is erased",
        ["bug27621_1_highlighted.png", "bug27621_1_erased.png"],
        |run| Ok(run.session().erase(&["b"])?),
    )
}

/// `bug27621_2`: the detection highlight disappears with the cleared viewer.
pub(crate) fn detected_then_cleared() -> Result<(), Box<dyn Error>> {
    detected_then_hidden(
        "CR27621_2: Highlighting with color remains visible when shape is erased",
        ["bug27621_2_highlighted.png", "bug27621_2_erased.png"],
        |run| Ok(run.session().clear()?),
    )
}

/// `bug26870`: a deactivated selection mode is recomputed with the redisplayed shape.
pub(crate) fn deactivated_mode_recomputed() -> Result<(), Box<dyn Error>> {
    banner(&["============"]);
    banner(&[
        "CR26870: Visualization - deactivated selections are not updated after object's re-computation",
    ]);
    banner(&["============", ""]);
    let mut run = SessionRun::new()?;
    run.session()
        .display("b", &box_sized([1., 1., 1.])?, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_object_selection_mode("b", 2, true)?;
    run.session().set_object_selection_mode("b", 4, true)?;
    run.session().set_object_selection_mode("b", 4, false)?;
    run.session()
        .display("b", &box_sized([3., 3., 3.])?, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().set_selection_mode(4, true)?;
    run.session().move_cursor(80, 265)?;
    if run.session().pixel_rgba_name(90, 206)? == "CYAN 1" {
        println!(
            "ERROR: previously computed entities are highlighted! Selection mode 4 was not updated after redisplaying the box."
        );
    }
    if run.session().pixel_rgba_name(29, 216)? == "YELLOW 1" {
        println!(
            "ERROR: the box's face is not highlighted! Selection mode 4 was not updated after redisplaying the box."
        );
    }
    run.dump("bug26870.png")?;
    run.finish()
}

/// `bug30717`: an erased object's face selection highlight in another mode disappears.
pub(crate) fn erased_selection_presentation() -> Result<(), Box<dyn Error>> {
    banner(&["============="]);
    banner(&[
        "0030717: Visualization, AisInteractiveContext::Erase() - selection presentation in a mode other than the default after hiding an object",
    ]);
    banner(&["============="]);
    let b = box_sized([10., 10., 10.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let request = DisplayRequest {
        display_mode: Some(1),
        highlight_mode: Some(1),
        ..DisplayRequest::default()
    };
    run.session().display("b", &b, &request)?;
    run.session().set_axonometric()?;
    run.session().fit()?;
    run.session().set_object_selection_mode("b", 4, true)?;
    run.session().move_cursor(250, 250)?;
    run.session().select_point(250, 250)?;
    run.session().erase(&["b"])?;
    if run.session().pixel_color_name(300, 300)? != "BLACK" {
        println!("Error: hidden face is highlighted");
    }
    run.dump("bug30717.png")?;
    run.finish()
}
