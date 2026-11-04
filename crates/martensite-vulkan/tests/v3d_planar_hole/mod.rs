//! The original shaded face and golden pixel assertion use the same healed procedural shape.
#[cfg(test)]
mod cpu;
mod geometry;
use crate::{
    case_view::viewer_session::{DisplayRequest, ViewerSession},
    session_run::{SessionRun, banner},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
type Outcome = Result<(), Box<dyn std::error::Error>>;
pub(super) fn display(session: &mut ViewerSession, shape: &Shape) -> Result<(), Failure> {
    session.display("result", shape, &DisplayRequest::default())?;
    session.set_object_display_mode("result", 1)?;
    session.fit()
}
/// Build both literal wires and repair/display their actual face; `planar_face_triangulation()?`.
pub(crate) fn planar_face_triangulation() -> Outcome {
    banner(&[
        "============",
        "OCC23219: Triangulation is not built on a planar face",
        "============",
        "",
    ]);
    let shape = geometry::healed_face()?;
    let mut run = SessionRun::new()?;
    display(run.session(), &shape)?;
    run.session().check_color(200, 200, [0.78, 0.55, 0.09])?;
    run.dump("bug23219.png")?;
    run.finish()
}
