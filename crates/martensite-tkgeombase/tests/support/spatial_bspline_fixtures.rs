//! Exact weighted spatial spline inputs from geometry/bsplinecurve.
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg3d::geom::GeomBSplineCurve;
use martensite_tkmath::gp::Point3;

fn weighted(
    poles: &[Point3],
    weights: &[f64],
    knots: &[f64],
    mults: &[i32],
) -> Handle<GeomBSplineCurve> {
    Handle::new(
        GeomBSplineCurve::with_weights(
            &Array1::from_shared(poles, 1),
            &Array1::from_shared(weights, 1),
            &Array1::from_shared(knots, 1),
            &Array1::from_shared(mults, 1),
            3,
            false,
        )
        .unwrap(),
    )
}
/// Original line on [-1,1]; `collinear_cubic()`.
pub fn collinear_cubic() -> Handle<GeomBSplineCurve> {
    weighted(
        &[
            Point3::new(0., 0., 0.),
            Point3::new(1., 0., 0.),
            Point3::new(2., 0., 0.),
            Point3::new(3., 0., 0.),
        ],
        &[1.; 4],
        &[-1., 1.],
        &[4, 4],
    )
}
fn position_poles() -> [Point3; 7] {
    [
        Point3::new(0., 0., 0.),
        Point3::new(1., 2., 0.),
        Point3::new(2., 2., 0.),
        Point3::new(2., 2., 0.),
        Point3::new(3., 2., 0.),
        Point3::new(4., 2., 0.),
        Point3::new(5., 4., 0.),
    ]
}
/// Initial positional tangent fixture A3 or A5; `moving_position(false)`.
pub fn moving_position(rational: bool) -> Handle<GeomBSplineCurve> {
    if rational {
        return moving_tangent(true);
    }
    weighted(
        &position_poles(),
        &[1.; 7],
        &[0., 1., 2., 3., 4.],
        &[4, 1, 1, 1, 4],
    )
}
fn tangent_poles() -> [Point3; 7] {
    [
        Point3::new(0., 0., 0.),
        Point3::new(1., 0.5, 0.),
        Point3::new(2., 1., 0.),
        Point3::new(2., 2., 0.),
        Point3::new(3., 1.5, 0.),
        Point3::new(4., 1.5, 0.),
        Point3::new(5., 2., 0.),
    ]
}
/// Initial tangent fixture A7 or A9; `moving_tangent(true)`.
pub fn moving_tangent(rational: bool) -> Handle<GeomBSplineCurve> {
    let weights = if rational {
        [0.8, 0.6, 0.5, 1., 1., 1., 1.]
    } else {
        [1.; 7]
    };
    weighted(
        &tangent_poles(),
        &weights,
        &[0., 1., 2., 3., 4.],
        &[4, 1, 1, 1, 4],
    )
}
fn one_knot_poles(last_x: f64) -> [Point3; 5] {
    [
        Point3::new(0., 0., 0.),
        Point3::new(1., 2., 0.),
        Point3::new(2., 3., 0.),
        Point3::new(3., 2., 0.),
        Point3::new(last_x, 0., 0.),
    ]
}
/// Original single-interior-knot fixture; `one_knot(false, 1.0)`.
pub fn one_knot(rational: bool, last_x: f64) -> Handle<GeomBSplineCurve> {
    let weights = if rational {
        [1., 20., 3., 0.04, 5.]
    } else {
        [1.; 5]
    };
    weighted(&one_knot_poles(last_x), &weights, &[0., 3., 4.], &[4, 1, 4])
}
