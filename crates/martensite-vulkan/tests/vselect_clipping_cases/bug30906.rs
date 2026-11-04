//! Local override on b1 must leave b2 governed by the global view plane.
use super::bug30777::display_point;
use super::prelude::*;

const BUG30906_BANNER: &[&str] = &[
    "=============",
    "0030906: Visualization, SelectMgrViewerSelector - Object clipping planes overrides View clipping plane for next objects",
    "=============",
];

pub(crate) fn override_does_not_leak() -> Result<(), Box<dyn Error>> {
    banner(BUG30906_BANNER);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    display_pair(run.session())?;
    run.session().fit()?;
    run.session().show_view_trihedron(&Default::default())?;
    display_point(run.session(), "p0", [0.; 3])?;
    let _planes = override_planes(run.session())?;
    run.session().move_cursor(200, 200)?;
    check_override_pixels(run.session())?;
    run.dump("bug30906.png")?;
    run.finish()
}

pub(super) fn display_pair(session: &mut ViewerSession) -> Result<(), Failure> {
    for (name, origin) in [("b1", [1., 1., 0.]), ("b2", [3., 0., 0.])] {
        let shape = box_at(origin, [1., 2., 3.])?;
        session.display(name, &shape, &shaded_highlighted())?;
    }
    Ok(())
}

pub(super) fn override_planes(session: &ViewerSession) -> Result<ClippingPlanes, Failure> {
    let mut planes = ClippingPlanes::new(session.view());
    planes.ensure("c1")?;
    planes.equation("c1", [0., 1., 0., 1.])?;
    object_plane(&mut planes, session, "c1", "b1", true)?;
    planes.ensure("c2")?;
    planes.equation("c2", [0., 1., 0., -1.])?;
    planes.on_view("c2", session.view())?;
    Ok(planes)
}

fn check_override_pixels(session: &ViewerSession) -> Result<(), Failure> {
    pixel(
        session,
        250,
        200,
        "DARKGOLDENROD",
        "Error: selection of clipped object",
    )?;
    pixel(session, 150, 200, "CYAN2", "Error: object is not selected")?;
    Ok(())
}
