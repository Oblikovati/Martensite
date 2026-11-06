//! Isoline setup uses the authored periodic curve and copies each object aspect.
use super::*;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::Array1;
use martensite_tkg3d::geom::GeomBSplineCurve;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::Graphic3dAspectsMembers;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeWire,
};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
use martensite_tkv3d::prs3d::Prs3dIsoAspect;

fn periodic_curve() -> Result<Handle<GeomBSplineCurve>, Failure> {
    let pole_buffer = [
        Point3::new(0., 0., 0.),
        Point3::new(1., 2., 0.),
        Point3::new(2., 0., 0.),
    ];
    let poles = Array1::from_shared(&pole_buffer, 1);
    let weights = Array1::from_shared(&[1.; 3], 1);
    let knots = Array1::from_shared(&[0., 1., 2., 3.], 1);
    let mults = Array1::from_shared(&[1; 4], 1);
    let curve = Handle::new(GeomBSplineCurve::with_weights(
        &poles, &weights, &knots, &mults, 2, true,
    )?);
    Ok(curve)
}
pub(super) fn periodic_face() -> Result<Shape, Failure> {
    let curve = periodic_curve()?;
    let edge = BRepBuilderAPIMakeEdge::from_curve(&curve.cast())?.edge()?;
    let wire = BRepBuilderAPIMakeWire::from_edge(&edge)?.wire()?;
    Ok(BRepBuilderAPIMakeFace::from_wire(&wire, false)?
        .face()?
        .into_shape())
}
fn copied_iso(aspect: Handle<Prs3dIsoAspect>) -> Result<Handle<Prs3dIsoAspect>, Failure> {
    let line = aspect.aspect();
    Ok(Handle::new(Prs3dIsoAspect::new(
        line.interior_color(),
        line.line_kind(),
        f64::from(line.width()),
        50,
    )?))
}
pub(super) fn configure_isolines(session: &ViewerSession) -> Result<(), Failure> {
    session.context().set_iso_on_plane(true);
    let object = session.object("ff").cloned().unwrap_or_default();
    let drawer = object.attributes();
    drawer.set_u_iso_aspect(copied_iso(drawer.u_iso_aspect())?);
    drawer.set_v_iso_aspect(copied_iso(drawer.v_iso_aspect())?);
    session
        .context()
        .set_local_attributes(&object, &drawer, false)?;
    session.context().redisplay(&object, false, false)?;
    session.update_view(|_| Ok(()))
}
const ISOLINE_BANNER: [&str; 4] = [
    "============",
    "0031688: Visualization - Wrong ISO lines for a face created from BSpline",
    "============",
    "",
];
/// Full `bug31688`: all three pixels outside the authored profile must remain black.
pub(crate) fn periodic_face_isolines() -> Outcome {
    banner(&ISOLINE_BANNER);
    let face = periodic_face()?;
    let mut run = SessionRun::new()?;
    run.session().display("ff", &face, &Default::default())?;
    configure_isolines(run.session())?;
    run.session().fit()?;
    let mut wrong = false;
    for x in [365, 366, 367] {
        wrong |= run.session().pixel_color_name(x, 200)? != "BLACK";
    }
    if wrong {
        println!("Error: isolines are not expected here");
    }
    run.dump("bug31688.png")?;
    run.finish()
}
