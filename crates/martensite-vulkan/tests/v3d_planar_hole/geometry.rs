//! Surface-wire Inside=true and the unreversed added inner wire preserve the source topology.
mod iw_0;
mod iw_1;
mod iw_2;
mod ow_0;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Shape, Wire},
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkshhealing::{
    shape_extend::{BasicMsgRegistrator, ShapeExtendMsgRegistrator},
    shape_fix::{FixRoot, ShapeFixShape},
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
};
pub(super) fn source_wires() -> Result<[Wire; 2], Failure> {
    let outer = polyline(ow_0::OW_0.iter())?;
    let inner = polyline(
        iw_0::IW_0
            .iter()
            .chain(iw_1::IW_1.iter())
            .chain(iw_2::IW_2.iter()),
    )?;
    Ok([outer, inner])
}
fn polyline<'p>(points: impl Iterator<Item = &'p [f64; 3]>) -> Result<Wire, Failure> {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for [x, y, z] in points {
        polygon.add_point(Point3::new(*x, *y, *z))?;
    }
    polygon.wire()
}
pub(super) fn healed_face() -> Result<Shape, Failure> {
    let [outer, inner] = source_wires()?;
    let plane = GeomPlane::from_point(Point3::new(0., 0., 0.), Direction3::new(0., 0., 1.)?)?;
    let surface: Handle<dyn GeomSurface> = Handle::new(plane).cast();
    let mut maker = BRepBuilderAPIMakeFace::from_surface_wire(&surface, &outer, true)?;
    let mut face = maker.shape()?.clone();
    BRepBuilder::default().add(&mut face, inner.as_ref())?;
    fix_shape(face)
}
fn fix_shape(face: Shape) -> Result<Shape, Failure> {
    let fixer = ShapeFixShape::default();
    let messages: Handle<dyn BasicMsgRegistrator> =
        Handle::new(ShapeExtendMsgRegistrator::new()).cast();
    fixer.set_msg_registrator(&messages);
    fixer.init(face);
    fixer.perform()?;
    Ok(fixer.shape())
}
