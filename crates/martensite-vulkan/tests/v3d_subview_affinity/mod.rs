//! The source query observes real sensitive entity types; visibility is not inferred from names.
#[cfg(test)]
mod cpu;
mod scene;
use crate::session_run::{SessionRun, banner};
use martensite_tkv3d::{ais::AisRotationMode, std_select::StdSelectTypeOfSelectionImage};
type Outcome = Result<(), Box<dyn std::error::Error>>;
/// Hide the sphere only in its authored subview and execute both dumps; `subview_selection_affinity()?`.
pub(crate) fn subview_selection_affinity() -> Outcome {
    heading();
    let mut run = SessionRun::with_view_size(1024, 512)?;
    scene::prepare(run.session())?;
    run.session().activate_view(1)?;
    run.session().hide_in_active_view("s")?;
    run.session()
        .set_camera_rotation_mode(AisRotationMode::PickLast);
    run.session().move_cursor(100, 100)?;
    query_hidden_sphere(run.session())?;
    run.selection_dump(
        "bug32978_sel.png",
        StdSelectTypeOfSelectionImage::COLORED_ENTITY,
    )?;
    run.dump("bug32978_vis.png")?;
    run.finish()
}

fn heading() {
    banner(&[
        "============",
        "0031965: Visualization - AisInteractiveContext::HilightWithColor() ignores passed highlight style",
        "============",
        "",
    ]);
}
fn query_hidden_sphere(
    session: &crate::case_view::viewer_session::ViewerSession,
) -> Result<(), martensite_tkernel::standard::Failure> {
    if session
        .picked_entity_types()?
        .iter()
        .any(|name| name.contains("Select3DSensitiveSphere"))
    {
        println!("Error: sphere should NOT be detected");
    }
    Ok(())
}
