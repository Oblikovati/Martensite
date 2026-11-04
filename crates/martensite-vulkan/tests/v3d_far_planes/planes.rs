use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape};
pub(super) fn face(z: f64, bounds: (f64, f64, f64, f64)) -> Result<Shape, Failure> {
    let plane = GeomPlane::from_point(Point3::new(0., 0., z), Direction3::new(0., 0., 1.)?)?;
    let surface: Handle<dyn GeomSurface> = Handle::new(plane).cast();
    let mut maker =
        BRepBuilderAPIMakeFace::from_surface_bounds(&surface, bounds, precision::confusion())?;
    Ok(maker.shape()?.clone())
}
pub(super) fn display(session: &mut ViewerSession, min: f64, max: f64) -> Result<(), Failure> {
    let first = face(min, (-1., 0., -1., 0.))?;
    let second = face(max, (0., 1., 0., 1.))?;
    session.clear()?;
    session.display_many(
        &[("f1", &first), ("f2", &second)],
        &DisplayRequest::default(),
    )?;
    session.set_standard_projection(martensite_tkv3d::v3d::V3dTypeOfOrientation::Zpos)?;
    session.fit()
}
