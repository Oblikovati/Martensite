//! The protected defined `Geom2dBezierCurve::init` reuse path on a nonrational curve.
use martensite_tkernel::ncollection::Array1;
use martensite_tkg2d::geom2d::Geom2dBezierCurve;
use martensite_tkmath::gp::Point2;

fn poles(count: i32) -> Array1<'static, Point2> {
    let mut result = Array1::with_bounds(1, count).unwrap();
    for i in 1..=count {
        result[i] = Point2::new(f64::from(i), f64::from(i * i));
    }
    result
}

fn weights(values: &[f64]) -> Array1<'static, f64> {
    let mut result = Array1::with_bounds(1, values.len() as i32).unwrap();
    result.as_mut_slice().copy_from_slice(values);
    result
}

/// Reinitialization owns the supplied weights instead of mutating shared unit weights.
/// A later nonrational curve must still start with unit weights.
#[test]
fn same_count_weighted_init_of_a_nonrational_curve_stores_the_weights() {
    let curve = Geom2dBezierCurve::new(&poles(3)).unwrap();
    assert!(!curve.is_rational());
    curve
        .init(&poles(3), Some(&weights(&[1.0, 2.0, 3.0])))
        .unwrap();
    assert!(curve.is_rational());
    let stored: Vec<f64> = (1..=3).map(|i| curve.weight(i).unwrap()).collect();
    assert_eq!(stored, [1.0, 2.0, 3.0]);
    assert_eq!(curve.pole(3).unwrap(), Point2::new(3.0, 9.0));
    let fresh = Geom2dBezierCurve::new(&poles(3)).unwrap();
    assert_eq!(fresh.weight(2).unwrap(), 1.0);
}

/// Constant weights given to `init` are not rational: defined replaces them by unit weights.
#[test]
fn constant_weights_leave_the_curve_nonrational_with_unit_weights() {
    let curve = Geom2dBezierCurve::new(&poles(3)).unwrap();
    curve
        .init(&poles(3), Some(&weights(&[4.0, 4.0, 4.0])))
        .unwrap();
    assert!(!curve.is_rational());
    assert_eq!(curve.weight(1).unwrap(), 1.0);
}
