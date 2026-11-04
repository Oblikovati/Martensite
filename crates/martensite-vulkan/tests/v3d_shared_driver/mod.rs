//! Viewer activation restores its actual context before rendering the authored pixel query.
#[cfg(test)]
mod cpu;
use crate::{
    case_view::viewer_session::{DisplayRequest, ViewerSession},
    primitive_shapes::box_at,
    session_run::{SessionRun, banner},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::Failure,
    storage::format_general,
};
type Outcome = Result<(), Box<dyn std::error::Error>>;
pub(super) fn source_boxes() -> Result<[Shape; 4], Failure> {
    Ok([
        box_at([0.; 3], [1., 2., 3.])?,
        box_at([0., 3., 0.], [3., 1., 2.])?,
        box_at([0.; 3], [3., 1., 2.])?,
        box_at([0., 3., 0.], [1., 2., 3.])?,
    ])
}
/// Render both independent viewer owners after their method switch; `shared_driver_views()?`.
pub(crate) fn shared_driver_views() -> Outcome {
    banner(&[
        "============",
        "0028890: Visualization - After closing all views and then display the view again, just the first view has object(s) displayed",
        "============",
        "",
    ]);
    let shapes = source_boxes()?;
    let mut run = SessionRun::new()?;
    prepare(run.session(), &shapes)?;
    let first = queried_image(&mut run, 0, "bug28890_1.png")?;
    let second = queried_image(&mut run, 1, "bug28890_2.png")?;
    if first != second {
        println!("Error: colors should be equal");
    }
    run.finish()
}
pub(super) fn prepare(session: &mut ViewerSession, shapes: &[Shape; 4]) -> Result<(), Failure> {
    viewer_scene(session, [("b1_1", &shapes[0]), ("b1_2", &shapes[1])])?;
    session.open_viewer()?;
    viewer_scene(session, [("b2_1", &shapes[2]), ("b2_2", &shapes[3])])
}
fn viewer_scene(session: &mut ViewerSession, shapes: [(&str, &Shape); 2]) -> Result<(), Failure> {
    session.show_view_trihedron(&Default::default())?;
    session.display_many(&shapes, &DisplayRequest::default().with_display_mode(1))?;
    session.fit()?;
    session.set_color(shapes[0].0, Color::named(NamedColor::RED))?;
    session.set_color(shapes[1].0, Color::named(NamedColor::GREEN))
}
fn queried_image(
    run: &mut SessionRun,
    index: usize,
    image: &'static str,
) -> Result<String, Box<dyn std::error::Error>> {
    run.session().activate_view(index)?;
    crate::view_lifecycle_commands::switch_to_raytrace(run.session())?;
    let color = run.session().pixel_color(150, 250)?;
    let text = [color.red(), color.green(), color.blue()]
        .map(|v| format_general(v, 17))
        .join(" ");
    run.dump(image)?;
    Ok(text)
}
