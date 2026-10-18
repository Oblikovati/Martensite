//! Reuses the same retained protected initialization scenarios in both point dimensions.
use martensite_tkernel::ncollection::Array1;
use martensite_tkg2d::geom2d::{Geom2dBezierCurve, Geom2dCurve};
use martensite_tkg3d::geom::{GeomBezierCurve, GeomCurve};
use martensite_tkmath::gp::{Point2, Point3};

fn weights(lower: i32, values: &[f64]) -> Array1<'static, f64> {
    let mut result = Array1::with_bounds(lower, lower + values.len() as i32 - 1).unwrap();
    result.as_mut_slice().copy_from_slice(values);
    result
}

macro_rules! dimension_probe {
    ($module:ident, $curve:ty, $point:ty, $create:expr) => {
        mod $module {
            use super::*;
            fn poles(count: i32, end: f64) -> Array1<'static, $point> {
                let mut result = Array1::with_bounds(5, 4 + count).unwrap();
                for i in 0..count {
                    result[5 + i] = ($create)(if i == count - 1 { end } else { f64::from(i) });
                }
                result
            }
            fn snapshot(curve: &$curve) -> String {
                format!(
                    "{} {} {} {} {} {}\n",
                    curve.nb_poles(),
                    curve.degree(),
                    i32::from(curve.is_rational()),
                    i32::from(curve.is_closed().unwrap()),
                    curve.pole(1).unwrap().x(),
                    curve.weight(1).unwrap()
                )
            }
            pub(super) fn receipt() -> String {
                let initial = weights(1, &[1.0, 2.0]);
                let curve = <$curve>::with_weights(&poles(2, 2.0), &initial).unwrap();
                let mut observed = failed_weights(&curve, &initial);
                observed.push_str(&weighted_updates(&curve));
                observed.push_str(&singleton_and_empty(&curve));
                curve.init(&poles(2, 1.0e-8), None).unwrap();
                observed.push_str(&format!("{}\n", i32::from(curve.is_closed().unwrap())));
                observed
            }
            fn failed_weights(curve: &$curve, initial: &Array1<'_, f64>) -> String {
                let three = poles(3, 0.0);
                let error = curve.init(&three, Some(initial)).unwrap_err();
                format!(
                    "{} {} {} {}\n",
                    error.exception_type(),
                    curve.nb_poles(),
                    i32::from(curve.is_rational()),
                    i32::from(curve.is_closed().unwrap())
                )
            }
            fn weighted_updates(curve: &$curve) -> String {
                let three = poles(3, 0.0);
                curve
                    .init(&three, Some(&weights(8, &[-1.0, 2.0, 3.0])))
                    .unwrap();
                let first = snapshot(curve);
                curve.init(&three, Some(&weights(8, &[0.0; 3]))).unwrap();
                format!("{first}{}", snapshot(curve))
            }
            fn singleton_and_empty(curve: &$curve) -> String {
                curve.init(&poles(1, 7.0), None).unwrap();
                let before = snapshot(curve);
                let error = curve.init(&Array1::default(), None).unwrap_err();
                format!("{before}{}\n{}", error.exception_type(), snapshot(curve))
            }
            #[test]
            fn successful_init_invalidates_derivative_cache() {
                let curve = <$curve>::new(&poles(2, 2.0)).unwrap();
                assert_eq!(curve.resolution(1.0).unwrap(), 0.5);
                curve.init(&poles(2, 8.0), None).unwrap();
                assert_eq!(curve.resolution(1.0).unwrap(), 0.125);
            }
            #[test]
            fn failed_init_preserves_cached_derivative_and_existing_weights() {
                let curve =
                    <$curve>::with_weights(&poles(2, 2.0), &weights(1, &[1.0, 2.0])).unwrap();
                let cached = curve.resolution(1.0).unwrap();
                assert!(
                    curve
                        .init(&poles(2, 8.0), Some(&weights(1, &[1.0])))
                        .is_err()
                );
                assert_eq!(curve.resolution(1.0).unwrap(), cached);
                assert_eq!(curve.weight(2).unwrap(), 2.0);
                assert_eq!(curve.pole(2).unwrap().x(), 8.0);
            }
        }
    };
}

dimension_probe!(plane, Geom2dBezierCurve, Point2, |x| Point2::new(x, 0.0));
dimension_probe!(space, GeomBezierCurve, Point3, |x| Point3::new(x, 0.0, 0.0));

/// Same-size weighted initialization stores the caller weights and makes the
/// curve rational without mutating the reusable unit-weight table.
#[test]
fn same_count_weighted_init_of_a_nonrational_curve_stores_the_weights() {
    let curve = GeomBezierCurve::new(&space_poles(3)).unwrap();
    assert!(!curve.is_rational());
    curve
        .init(&space_poles(3), Some(&weights(1, &[1.0, 2.0, 3.0])))
        .unwrap();
    assert!(curve.is_rational());
    let stored: Vec<f64> = (1..=3).map(|i| curve.weight(i).unwrap()).collect();
    assert_eq!(stored, [1.0, 2.0, 3.0]);
    let fresh = GeomBezierCurve::new(&space_poles(3)).unwrap();
    assert_eq!(fresh.weight(2).unwrap(), 1.0);
}

fn space_poles(count: i32) -> Array1<'static, Point3> {
    let mut result = Array1::with_bounds(1, count).unwrap();
    for i in 1..=count {
        result[i] = Point3::new(f64::from(i), f64::from(i * i), 0.0);
    }
    result
}

#[test]
fn protected_initialization_preserves_dimensions_and_failure_order() {
    let observed = format!("2d\n{}3d\n{}", plane::receipt(), space::receipt());
    assert_eq!(observed, include_str!("fixtures/bezier-initialization.txt"));
}
