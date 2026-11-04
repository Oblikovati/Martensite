//! The quadrangle renders a face but remains selectable by its actual primitive-array points.
use super::commands::{display_priority, owner_priority, persistent_existing, white_dump};
use super::*;

/// Keep the box and corner-selectable quad as distinct objects; `quad_scene(session)?`.
pub(super) fn quad_scene(session: &mut ViewerSession) -> Result<(), Failure> {
    let shape = objects::first_box()?;
    objects::display_box(session, "box_1", &shape)?;
    material_color(session, "box_1", NamedColor::DEEPPINK2)?;
    objects::display_array(session, "quad1", objects::quad_array()?)?;
    persistent_existing(session, "quad1", Some(1))?;
    material_color(session, "quad1", NamedColor::PALEGREEN2)
}

fn default_quad(run: &mut SessionRun) -> Outcome {
    run.dump("bug30484_4_box_quad_undefined.png")?;
    run.session().move_cursor(100, 100)?;
    run.dump("bug30484_4_box_quad_boxhighlight.png")?;
    run.session().move_cursor(376, 100)?;
    run.dump("bug30484_4_box_quad_quadhighlight.png")
}

fn focused_quad(run: &mut SessionRun) -> Outcome {
    run.session().move_cursor(0, 0)?;
    display_priority(run.session(), "box_1", 6)?;
    display_priority(run.session(), "quad1", 5)?;
    owner_priority(run.session(), "box_1", 5)?;
    owner_priority(run.session(), "quad1", 6)?;
    white_dump(
        run,
        (175, 99),
        (300, 200),
        "Error: quad primitive should be highlighted",
        "bug30484_4_box_quad_selectionpriority_focused.png",
    )
}

/// Complete box-versus-point-sensitive quadrangle original; `priority_quad()`.
pub(crate) fn priority_quad() -> Outcome {
    let mut run = priority_begin()?;
    quad_scene(run.session())?;
    default_quad(&mut run)?;
    focused_quad(&mut run)?;
    run.finish()
}
