//! Actual clipped sphere detection is checked at the original pixel and absolute tolerance.
use super::prelude::*;
use crate::primitive_shapes::sphere;
use crate::session_run::check_point;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkg3d::geom::GeomCartesianPoint;
use martensite_tkv3d::ais::AisPoint;

const BUG30777_BANNER: &[&str] = &[
    "=============",
    "0030777: Visualization - Incorrect selection/highlighting of clipped objects",
    "=============",
];

pub(crate) fn clipped_sphere_point() -> Result<(), Box<dyn Error>> {
    banner(BUG30777_BANNER);
    let mut run = SessionRun::new()?;
    sphere_scene(run.session())?;
    let mut planes = ClippingPlanes::new(run.session().view());
    planes.ensure("p1")?;
    object_plane(&mut planes, run.session(), "p1", "s", false)?;
    planes.equation("p1", [0., 1., 0., 0.])?;
    run.session().move_cursor(200, 200)?;
    let point = run.session().detected_point()?;
    display_point(run.session(), "pp", point)?;
    run.session()
        .set_color("pp", Color::named(NamedColor::RED))?;
    check_point("p", point, [-0.34896, 9.99014, 0.27411], 0.0001);
    run.dump("bug30777.png")?;
    run.finish()
}

pub(super) fn display_point(
    session: &mut ViewerSession,
    name: &str,
    [x, y, z]: [f64; 3],
) -> Result<(), Failure> {
    let geometry = Handle::new(GeomCartesianPoint::new(x, y, z));
    let point = AisPoint::new(&geometry.cast())?;
    session.display_object(name, point.cast())
}

pub(super) fn sphere_scene(session: &mut ViewerSession) -> Result<(), Failure> {
    session.clear()?;
    session.set_axonometric()?;
    session.display(
        "s",
        &sphere(10.)?,
        &DisplayRequest::default().with_display_mode(1),
    )?;
    session.fit()?;
    session.set_standard_projection(V3dTypeOfOrientation::ZUP_FRONT)?;
    Ok(())
}
