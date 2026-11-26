use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCurve, GeomEllipse, GeomSurface, GeomSurfaceOfLinearExtrusion};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

pub(super) fn extrusion(scale: f64, phase: f64, height: f64) -> Shape {
    extrusion_ratio(scale, 0.6, phase, height)
}
pub(super) fn extrusion_ratio(scale: f64, ratio: f64, phase: f64, height: f64) -> Shape {
    let surface = elliptic_surface(scale, ratio);
    let first = std::f64::consts::FRAC_PI_2 + phase;
    BRepBuilderAPIMakeFace::from_surface_bounds(
        &surface,
        (first, first + std::f64::consts::PI, 0., height),
        1e-9,
    )
    .unwrap()
    .face()
    .unwrap()
    .into_shape()
}

fn elliptic_surface(scale: f64, ratio: f64) -> Handle<dyn GeomSurface> {
    let direction = Direction3::new(0., 0., 1.).unwrap();
    let axis = Axis2::new(Point3::new(4., 1.5, 0.), direction).unwrap();
    let curve: Handle<dyn GeomCurve> =
        Handle::new(GeomEllipse::new(axis, scale, ratio * scale).unwrap()).cast();
    Handle::new(GeomSurfaceOfLinearExtrusion::new(&curve, direction).unwrap()).cast()
}
