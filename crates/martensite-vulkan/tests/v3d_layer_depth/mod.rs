//! Keep thin boxes, material inheritance and literal layer1 offset commands in source order.
#[cfg(test)]
mod cpu;
mod layers;
mod objects;
use crate::{
    case_view::viewer_session::ViewerSession,
    session_run::{SessionRun, banner},
};
use martensite_tkernel::standard::Failure;
type Outcome = Result<(), Box<dyn std::error::Error>>;
/// Draw the authored overlap scene with explicit depth policies; `overlapping_layers()?`.
pub(crate) fn overlapping_layers() -> Outcome {
    banner(&[
        "This test case handles a specific task:",
        "to draw 2d objects in same scene with 3d objects while 2d objects need to be drawn in specific order on 3d plane and overlap correctly with 3d objects.",
    ]);
    let mut run = SessionRun::new()?;
    prepare(run.session())?;
    run.session().fit()?;
    run.dump("bug24785.png")?;
    run.finish()
}
pub(super) fn prepare(session: &mut ViewerSession) -> Result<i32, Failure> {
    objects::thin_boxes(session)?;
    objects::sphere(session)?;
    let layer = layers::source_layer(session)?;
    session.update_context(|_| session.set_object_z_layer("s", layer))?;
    layers::depth_policies(session, layer)?;
    objects::crossing_box(session, layer)?;
    layers::offset(session)?;
    Ok(layer)
}
