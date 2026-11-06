//! The actual cylinder and its source face binding drive both global and local selection.
mod commands;
#[cfg(test)]
mod cpu;
use crate::{
    case_view::viewer_session::DisplayRequest,
    primitive_shapes::cylinder,
    session_run::{SessionRun, banner},
};
type Outcome = Result<(), Box<dyn std::error::Error>>;

/// Preserve all original camera changes and five snapshots of actual highlighting; `highlight_discretization()?`.
pub(crate) fn highlight_discretization() -> Outcome {
    heading();
    let mut run = SessionRun::with_view_size(912, 912)?;
    run.session().clear()?;
    run.session().set_display_mode(1)?;
    let shape = cylinder(1., 100.)?;
    run.session()
        .display("p", &shape, &DisplayRequest::default())?;
    run.session().fit()?;
    commands::camera(run.session())?;
    run.dump("bug25540_shaded_pres.png")?;
    run.session().move_cursor(100, 100)?;
    run.dump("bug25540_h_pres.png")?;
    local_highlight(&mut run, "bug25540_local_h_pres.png")?;
    colored_highlight(&mut run, &shape)?;
    run.finish()
}
fn local_highlight(run: &mut SessionRun, image: &'static str) -> Outcome {
    commands::single_mode(
        run.session(),
        martensite_tkv3d::ais::ais_shape_selection_mode(martensite_tkg3d::top_abs::ShapeEnum::Edge),
    )?;
    run.session().move_cursor(130, 80)?;
    run.dump(image)?;
    Ok(())
}
fn colored_highlight(run: &mut SessionRun, shape: &martensite_tkbrep::topods::Shape) -> Outcome {
    commands::color_source_face(run.session(), shape)?;
    commands::single_mode(run.session(), 0)?;
    run.session().move_cursor(100, 100)?;
    run.dump("bug25540_h_sub_pres.png")?;
    local_highlight(run, "bug25540_local_h_sub_pres.png")
}
fn heading() {
    banner(&[
        "============",
        "CR25540",
        "Check that the discretization of local selection,",
        "wireframe and shaded presentation of shape is identical.",
        "============",
        "",
    ]);
}
