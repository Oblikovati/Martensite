//! Selection recomputation sees the actual children after their parent's mode is reactivated.
use super::*;
use crate::primitive_shapes::{box_sized, preview_box_at};

fn child_shapes() -> Result<[martensite_tkbrep::topods::Shape; 3], Failure> {
    Ok([
        box_sized([10.; 3])?,
        preview_box_at([-100., -100., 0.], [200., 200., 0.])?,
        preview_box_at([100., -100., 0.], [200., 200., 0.])?,
    ])
}
fn plane_request() -> DisplayRequest {
    DisplayRequest {
        display_mode: Some(1),
        highlight_mode: Some(1),
        ..Default::default()
    }
}
pub(super) fn child_scene(session: &mut ViewerSession) -> Result<(), Failure> {
    let shapes = child_shapes()?;
    session.display(
        "b",
        &shapes[0],
        &DisplayRequest::default().with_display_mode(1),
    )?;
    session.display_many(&[("p1", &shapes[1]), ("p2", &shapes[2])], &plane_request())?;
    session.fit()
}
pub(super) fn attach_and_reactivate(session: &mut ViewerSession) -> Result<(), Failure> {
    let parent = session.object("b").cloned().unwrap_or_default();
    session.context().deactivate_mode(&parent, 0)?;
    session.erase(&["p1"])?;
    session.add_child("b", "p1")?;
    session.add_child("b", "p2")?;
    session.context().activate(&parent, 0, false)?;
    session.move_cursor(100, 200)
}
/// Full `bug31794`; no camera refit is added after the children are attached.
pub(crate) fn child_mode_reactivation() -> Outcome {
    banner(&[
        "============",
        "0031794: Visualization - Highlighting of AisShape works incorrectly when AddChild() used",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    child_scene(run.session())?;
    attach_and_reactivate(run.session())?;
    if run.session().pixel_color_name(100, 200)? != "BLACK" {
        println!("Error");
    }
    run.dump("bug31794.png")?;
    run.finish()
}
