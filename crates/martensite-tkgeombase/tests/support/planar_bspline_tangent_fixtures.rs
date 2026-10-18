//! Original hundred-step position/tangent sequences and explicit weighted inputs.
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve, Geom2dGeometry};
use martensite_tkmath::gp::{Point2, Vector2};

fn position_poles() -> [Point2; 7] {
    [
        Point2::new(0., 0.),
        Point2::new(1., 2.),
        Point2::new(2., 2.),
        Point2::new(2., 2.),
        Point2::new(3., 2.),
        Point2::new(4., 2.),
        Point2::new(5., 4.),
    ]
}
fn tangent_poles() -> [Point2; 7] {
    [
        Point2::new(0., 0.),
        Point2::new(1., 0.5),
        Point2::new(2., 1.),
        Point2::new(2., 2.),
        Point2::new(3., 1.5),
        Point2::new(4., 1.5),
        Point2::new(5., 2.),
    ]
}
fn original_position_curve(rational: bool) -> Handle<Geom2dBSplineCurve> {
    let (poles, weights) = if rational {
        (tangent_poles(), [0.8, 0.6, 0.5, 1., 1., 1., 1.])
    } else {
        (position_poles(), [1.; 7])
    };
    super::weighted_spline_with_weights(
        &poles,
        &weights,
        &[0., 1., 2., 3., 4.],
        &[4, 1, 1, 1, 4],
        3,
    )
}
fn original_tangent_curve(rational: bool) -> Handle<Geom2dBSplineCurve> {
    let weights = if rational {
        [1., 0.8, 1., 0.7, 0.6, 1., 1.]
    } else {
        [1.; 7]
    };
    super::weighted_spline_with_weights(
        &tangent_poles(),
        &weights,
        &[0., 1., 2., 3., 4.],
        &[4, 1, 1, 1, 4],
        3,
    )
}
/// Preserve all100 original position requests and the final deep copy; `position_sequence(false)`.
pub fn position_sequence(rational: bool) -> Handle<Geom2dBSplineCurve> {
    let curve = original_position_curve(rational);
    let original = curve.eval_d1(2.).unwrap();
    let mut y = original.point.y();
    for _ in 0..100 {
        let point = Point2::new(original.point.x(), y);
        let _status = curve
            .move_point_and_tangent(2., point, original.d1, 1e-5, 1, 1)
            .unwrap();
        y += 0.01; // Original loop updates after each requested point.
    }
    curve.copy().unwrap().cast()
}
/// Preserve all100 original tangent requests and the final deep copy; `tangent_sequence(true)`.
pub fn tangent_sequence(rational: bool) -> Handle<Geom2dBSplineCurve> {
    let curve = original_tangent_curve(rational);
    let original = curve.eval_d1(2.).unwrap();
    let mut dy = original.d1.y();
    let delta = if rational { 0.01 } else { 0.005 };
    for _ in 0..100 {
        let tangent = Vector2::new(original.d1.x(), dy);
        let _status = curve
            .move_point_and_tangent(2., original.point, tangent, 1e-5, 1, 1)
            .unwrap();
        dy += delta;
    }
    curve.copy().unwrap().cast()
}
