//! A point's own CYAN highlight remains distinct from global WHITE triangle highlights.
use super::commands::{display_priority, owner_priority, persistent_existing, pixel};
use super::*;

/// Preserve the initial and repeated display of the 2D point; `point_scene(session)?`.
pub(super) fn point_scene(session: &mut ViewerSession) -> Result<(), Failure> {
    super::arrays::triangle_scene(session)?;
    objects::screen_point(session)?;
    persistent_existing(session, "p1", Some(1))?;
    Ok(())
}

fn default_point(run: &mut SessionRun) -> Outcome {
    run.dump("bug30484_3_quad_point_undefined.png")?;
    run.session().move_cursor(175, 150)?;
    pixel(
        run,
        (175, 150),
        "CYAN",
        true,
        "Error: point should be highlighted",
    )?;
    run.dump("bug30484_3_quad_point_onpoint.png")?;
    run.session().move_cursor(150, 100)?;
    run.dump("bug30484_3_quad_point_ontri2.png")?;
    run.session().move_cursor(200, 200)?;
    run.dump("bug30484_3_quad_point_ontri1.png")
}

fn point_priorities(session: &ViewerSession) -> Result<(), Failure> {
    display_priority(session, "tri1", 4)?;
    display_priority(session, "tri2", 4)?;
    display_priority(session, "p1", 4)?;
    owner_priority(session, "tri1", 6)?;
    owner_priority(session, "tri2", 6)?;
    owner_priority(session, "p1", 5)
}

fn reduced_point(run: &mut SessionRun) -> Outcome {
    run.session().move_cursor(0, 0)?;
    point_priorities(run.session())?;
    run.session().move_cursor(175, 150)?;
    pixel(
        run,
        (175, 150),
        "CYAN",
        false,
        "Error: point should not be highlighted",
    )?;
    run.dump("bug30484_3_quad_point_changedpriorityfocus.png")?;
    run.session().move_cursor(0, 0)?;
    owner_priority(run.session(), "p1", 4)?;
    run.session().move_cursor(175, 150)?;
    run.dump("bug30484_3_quad_point_reducedpointpriority.png")
}

/// Complete point-versus-triangle priority original; `priority_point()`.
pub(crate) fn priority_point() -> Outcome {
    let mut run = priority_begin()?;
    point_scene(run.session())?;
    default_point(&mut run)?;
    reduced_point(&mut run)?;
    run.finish()
}
