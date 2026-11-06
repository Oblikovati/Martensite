//! Both display-priority and selection-priority phases, including overwritten image names.
use super::commands::{display_priority, owner_priority, pixel, white_dump};
use super::*;

/// Preserve the two boxes and their original creation/display order; `box_scene(session)?`.
pub(super) fn box_scene(session: &mut ViewerSession) -> Result<(), Failure> {
    let first = objects::first_box()?;
    let second = objects::second_box()?;
    objects::display_box(session, "box_1", &first)?;
    material_color(session, "box_1", NamedColor::DEEPPINK2)?;
    objects::display_box(session, "box_2", &second)?;
    material_color(session, "box_2", NamedColor::PALEGREEN2)
}

fn box_colors(run: &mut SessionRun) -> Outcome {
    pixel(
        run,
        (205, 170),
        "DARKSEAGREEN4",
        true,
        "Error: selected pixel should be green.",
    )?;
    pixel(
        run,
        (150, 170),
        "MAROON",
        true,
        "Error: selected pixel should be maroon.",
    )?;
    Ok(())
}

fn default_boxes(run: &mut SessionRun) -> Outcome {
    box_colors(run)?;
    white_dump(
        run,
        (205, 170),
        (205, 170),
        "Error: box_1 should be highlighted",
        "bug30484_1_boxes_undefined.png",
    )
}

fn displayed_boxes(run: &mut SessionRun) -> Outcome {
    run.session().move_cursor(0, 0)?;
    display_priority(run.session(), "box_1", 5)?;
    display_priority(run.session(), "box_2", 4)?;
    white_dump(
        run,
        (270, 170),
        (100, 170),
        "Error: box_1 should be highlighted",
        "bug30484_1_highlight_box1.png",
    )?;
    white_dump(
        run,
        (280, 170),
        (350, 170),
        "Error: box_2 should be highlighted",
        "bug30484_1_highlight_box2.png",
    )
}

fn selected_boxes(run: &mut SessionRun) -> Outcome {
    run.session().move_cursor(0, 0)?;
    owner_priority(run.session(), "box_1", 4)?;
    owner_priority(run.session(), "box_2", 5)?;
    white_dump(
        run,
        (180, 170),
        (350, 170),
        "Error: box_2 should be highlighted",
        "bug30484_1_highlight_box1.png",
    )?;
    white_dump(
        run,
        (170, 170),
        (100, 170),
        "Error: box_1 should be highlighted",
        "bug30484_1_highlight_box2.png",
    )
}

/// Complete two-box original with the source's repeated dump paths; `priority_boxes()`.
pub(crate) fn priority_boxes() -> Outcome {
    let mut run = priority_begin()?;
    box_scene(run.session())?;
    default_boxes(&mut run)?;
    displayed_boxes(&mut run)?;
    selected_boxes(&mut run)?;
    run.finish()
}
