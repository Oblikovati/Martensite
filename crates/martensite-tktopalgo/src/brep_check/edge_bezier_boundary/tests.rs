use super::*;
use martensite_tkbrep::brep::BRepCurve3D;
use martensite_tkernel::ncollection::Array1;
use martensite_tkg3d::geom::{GeomCurve, GeomGeometry};
use martensite_tkmath::gp::{Point2, Vector3};
use martensite_tkmath::toploc::Location;

fn point(u: f64, v: f64) -> Point3 {
    Point3::new(u, v, u * v)
}

fn patch() -> Handle<GeomBezierSurface> {
    let mut poles = [point(0., 0.), point(0., 1.), point(1., 0.), point(1., 1.)];
    let mut weights = [1., 0.75, 0.5, 1.];
    let poles = Array2::from_buffer(&mut poles, 2, 2).unwrap();
    let weights = Array2::from_buffer(&mut weights, 2, 2).unwrap();
    Handle::new(GeomBezierSurface::with_weights(&poles, &weights).unwrap())
}

fn rail(poles: [Point3; 2], weights: [f64; 2]) -> Handle<dyn Adaptor3dCurve> {
    adaptor(&literal_rail(poles, weights))
}

fn literal_rail(poles: [Point3; 2], weights: [f64; 2]) -> Handle<dyn GeomCurve> {
    Handle::new(
        GeomBezierCurve::with_weights(
            &Array1::from_shared(&poles, 1),
            &Array1::from_shared(&weights, 1),
        )
        .unwrap(),
    )
    .into_upcast()
}

fn adaptor(curve: &Handle<dyn GeomCurve>) -> Handle<dyn Adaptor3dCurve> {
    Handle::new(GeomAdaptorCurve::with_curve(curve).unwrap()).into_upcast()
}

fn uv(poles: [Point2; 2]) -> Handle<dyn Geom2dCurve> {
    Handle::new(Geom2dBezierCurve::new(&Array1::from_shared(&poles, 1)).unwrap()).into_upcast()
}

#[test]
fn proves_every_literal_rational_unit_boundary() {
    let surface = patch().into_upcast();
    for (a, b, weights) in [
        ([0., 0.], [1., 0.], [1., 0.5]),
        ([0., 1.], [1., 1.], [0.75, 1.]),
        ([0., 0.], [0., 1.], [1., 0.75]),
        ([1., 0.], [1., 1.], [0.5, 1.]),
    ] {
        let edge = rail([point(a[0], a[1]), point(b[0], b[1])], weights);
        let trace = uv([Point2::new(a[0], a[1]), Point2::new(b[0], b[1])]);
        assert!(identical_boundary(&edge, &surface, &trace));
    }
}

#[test]
fn defers_changed_controls_weights_and_trace_direction() {
    let surface = patch();
    let world = surface.clone().into_upcast();
    let a = point(0., 0.);
    let b = point(1., 0.);
    let trace = uv([Point2::new(0., 0.), Point2::new(1., 0.)]);
    let edge = rail([a, b], [1., 0.5]);
    assert!(!identical_boundary(
        &rail([a, b], [1., 0.6]),
        &world,
        &trace
    ));
    let reversed = uv([Point2::new(1., 0.), Point2::new(0., 0.)]);
    assert!(!identical_boundary(&edge, &world, &reversed));
    surface.set_pole(2, 1, Point3::new(1., 0., 0.001)).unwrap();
    assert!(!identical_boundary(&edge, &world, &trace));
}

#[test]
fn common_translation_cannot_hide_mutated_raw_control() {
    let mut placement = Transform3::default();
    placement.set_translation(Vector3::new(0., 0., 2_f64.powi(50)));
    let location = Location::new(placement);
    let patch = patch();
    let raw_surface = patch.clone().into_upcast();
    let world = patch.transformed(placement).unwrap().cast();
    let trace = uv([Point2::new(0., 0.), Point2::new(1., 0.)]);
    for (z, identical) in [(0., true), (0.001, false)] {
        let source = literal_rail([point(0., 0.), Point3::new(1., 0., z)], [1., 0.5]);
        let moved = source.get().unwrap().transformed(placement).unwrap().cast();
        let loaded = adaptor(&moved);
        let stored = BRepCurve3D::new(&source, &location);
        assert!(identical_boundary(&loaded, &world, &trace));
        assert_eq!(
            identical_located_boundary(&loaded, &world, &trace, &stored, &raw_surface, placement),
            identical
        );
    }
}
