//! The two complete original ray-axis regressions, preserving the source's A2 binding.
use super::{
    datum_objects,
    ray_command::pick_axis,
    source_checks::{check_no_axis_hits, check_points},
};
use crate::{
    case_view::viewer_session::DisplayRequest,
    primitive_shapes::{box_at, box_sized, sphere},
    session_run::{SessionRun, banner, check_point},
};
use martensite_tkservice::aspect::AspectTypeOfMarker;
use std::error::Error;

fn axis_banner() {
    banner(&[
        "========",
        "0032338: Visualization - provide straightforward interface for ray-picking",
        "========",
        "",
    ]);
}

/// All A1 geometry, ray picks, annotations and its image; `axis_a1()`.
pub(crate) fn axis_a1() -> Result<(), Box<dyn Error>> {
    axis_banner();
    let mut run = SessionRun::new()?;
    axis_a1_scene(&mut run)?;
    axis_a1_intersections(&mut run)?;
    run.dump("A1.png")?;
    run.finish()
}

fn axis_a1_intersections(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    let miss = pick_axis(run, [-4., 0., 6.], [0., 0., -1.], "sel_a0", false)?;
    check_no_axis_hits(&miss.text);
    let first = pick_axis(run, [0., 0., 6.], [0., 0., -1.], "sel_a1", false)?;
    check_points(
        &first.points,
        &[[0., 0., 5.], [0., 0., 4.], [0., 0., 2.]],
        0.001,
    );
    let second = pick_axis(run, [1., 0., 6.], [0., 0., -1.], "sel_a2", true)?;
    check_points(
        &second.points,
        &[[1., 0., 4.], [1., 0., 2.], [1., 0., 1.7320508075688776]],
        0.0001,
    );
    Ok(())
}

fn axis_a1_scene(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    let session = run.session();
    let shaded = DisplayRequest::default().with_display_mode(1);
    session.display("s", &sphere(2.)?, &shaded)?;
    session.set_transparency("s", 0.5)?;
    session.display("b", &box_at([-1., -1., 2.], [2., 2., 2.])?, &shaded)?;
    session.set_transparency("b", 0.5)?;
    datum_objects::line_axis(session, "a", [1., -2., 2.], [1., 2., 2.])?;
    datum_objects::point(session, "p", [0., 0., 5.], AspectTypeOfMarker::Plus, None)?;
    session.display_again("p")?;
    session.set_marker("p", AspectTypeOfMarker::O, 1.)?;
    session.set_axonometric()?;
    Ok(session.fit()?)
}

/// A2 retains both cursor moves and images, including its original point1 reuse; `axis_a2()`.
pub(crate) fn axis_a2() -> Result<(), Box<dyn Error>> {
    axis_banner();
    let mut run = SessionRun::new()?;
    axis_a2_scene(&mut run)?;
    run.session().move_cursor(324, 320)?;
    let point1 = run.session().detected_point()?;
    run.dump("A2_p1.png")?;
    check_point("point1", point1, [100., 200., 0.], 0.001);
    pick_axis(&mut run, [50., -100., 100.], [0., 1., 0.], "a", false)?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(324, 320)?;
    run.dump("A2_p2.png")?;
    // Source regexp parses point1 again; do not silently strengthen its historical predicate.
    check_point("point2", point1, [100., 200., 0.], 0.001);
    run.finish()
}

fn axis_a2_scene(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    let session = run.session();
    session.display(
        "b",
        &box_sized([100., 200., 300.])?,
        &DisplayRequest::default().with_display_mode(1),
    )?;
    session.fit()?;
    session.set_object_selection_mode("b", 1, true)?;
    Ok(session.set_object_selection_mode("b", 4, true)?)
}
