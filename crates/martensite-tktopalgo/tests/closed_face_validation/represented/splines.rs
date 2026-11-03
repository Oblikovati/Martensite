//! Stored array bits and layouts are checked after construction, never assumed.
use super::types::*;
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg2d::geom2d::Geom2dBSplineCurve;
use martensite_tkg3d::geom::GeomBSplineCurve;
use martensite_tkmath::gp::{Point2, Point3};
pub(super) fn curve3(input: SplineInput<3>) -> Handle<GeomBSplineCurve> {
    assert!(!input.evaluation_override);
    let points: Vec<_> = input.poles.iter().copied().map(point3).collect();
    let weights = floats(input.weights);
    let knots = floats(input.knots);
    let curve = GeomBSplineCurve::with_weights_checked(
        &Array1::from_shared(&points, input.lower_bounds[0]),
        &Array1::from_shared(&weights, input.lower_bounds[1]),
        &Array1::from_shared(&knots, input.lower_bounds[2]),
        &Array1::from_shared(input.multiplicities, input.lower_bounds[3]),
        input.degree,
        input.periodic,
        !input.rational,
    )
    .unwrap();
    Handle::new(curve)
}
pub(super) fn curve2(input: SplineInput<2>) -> Handle<Geom2dBSplineCurve> {
    assert!(!input.evaluation_override);
    let points: Vec<_> = input.poles.iter().copied().map(point2).collect();
    let weights = floats(input.weights);
    let knots = floats(input.knots);
    let curve = Geom2dBSplineCurve::with_weights(
        &Array1::from_shared(&points, input.lower_bounds[0]),
        &Array1::from_shared(&weights, input.lower_bounds[1]),
        &Array1::from_shared(&knots, input.lower_bounds[2]),
        &Array1::from_shared(input.multiplicities, input.lower_bounds[3]),
        input.degree,
        input.periodic,
    )
    .unwrap();
    Handle::new(curve)
}
fn floats(input: &[u64]) -> Vec<f64> {
    input.iter().copied().map(f64::from_bits).collect()
}

fn point3(bits: [u64; 3]) -> Point3 {
    let v = scalars(bits);
    Point3::new(v[0], v[1], v[2])
}
fn point2(bits: [u64; 2]) -> Point2 {
    let v = scalars(bits);
    Point2::new(v[0], v[1])
}
