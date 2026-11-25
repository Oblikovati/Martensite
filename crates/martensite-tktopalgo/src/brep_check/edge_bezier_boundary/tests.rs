use super::*;
use martensite_tkernel::ncollection::Array1;
use martensite_tkmath::gp::Point2;

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
    let curve = GeomBezierCurve::with_weights(
        &Array1::from_shared(&poles, 1),
        &Array1::from_shared(&weights, 1),
    )
    .unwrap();
    let adaptor = GeomAdaptorCurve::with_curve(&Handle::new(curve).into_upcast()).unwrap();
    Handle::new(adaptor).into_upcast()
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
