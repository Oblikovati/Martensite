//! Construct the actual GC trimmed surfaces and preserve sewing's nullable binding update.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    message::ProgressRange,
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkgeombase::gc::MakeTrimmedCone;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape, BRepBuilderAPISewing,
};
pub(super) fn faces() -> Result<[Shape; 3], Failure> {
    Ok([face(8., 16.)?, face(8., 0.)?, face(16., 8.)?])
}
fn face(first: f64, second: f64) -> Result<Shape, Failure> {
    let surface: Handle<dyn GeomSurface> = MakeTrimmedCone::from_points_radii(
        Point3::new(0., 0., 20.),
        Point3::new(0., 0., 45.),
        first,
        second,
    )?
    .value()?
    .cast();
    let mut maker = BRepBuilderAPIMakeFace::from_surface(&surface, precision::confusion())?;
    Ok(maker.shape()?.clone())
}
pub(super) fn sewn_bindings(shapes: &[Shape; 3]) -> Result<[Shape; 3], Failure> {
    Ok([sewn(&shapes[0])?, sewn(&shapes[1])?, sewn(&shapes[2])?])
}
pub(super) fn sewn(shape: &Shape) -> Result<Shape, Failure> {
    let mut sewing = BRepBuilderAPISewing::new(1.0e-7, true, true, true, false);
    sewing.set_same_parameter_mode(true);
    sewing.set_floating_edges_mode(false);
    sewing.set_face_mode(true);
    sewing.set_min_tolerance(precision::confusion());
    sewing.set_max_tolerance(precision::infinite());
    sewing.add(shape)?;
    sewing.perform(&ProgressRange::default())?;
    sewing.dump(&mut std::io::stdout())?;
    if sewing.sewed_shape().is_null() {
        return Ok(shape.clone());
    }
    Ok(sewing.sewed_shape().clone())
}
