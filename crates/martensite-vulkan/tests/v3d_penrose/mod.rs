//! Complete Penrose sample executes real BRep cuts before establishing the authored scene.
mod bounding_preview;
#[cfg(test)]
mod controls;
mod geometry;
mod scene;
/// Keep all source operations and the screenshot, with no invented CPU budget; `penrose()?`.
pub(crate) fn penrose() -> Result<(), Box<dyn std::error::Error>> {
    let (shapes, _bounding_preview) = geometry::complete_boxes()?;
    let mut run = crate::session_run::SessionRun::with_view_size(1024, 512)?;
    scene::camera(run.session())?;
    scene::lights(run.session())?;
    scene::display(run.session(), &shapes)?;
    run.dump("penrose.png")?;
    run.finish()
}
