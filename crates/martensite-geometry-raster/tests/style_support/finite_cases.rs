//! Range doubling is checked against straight lines rather than another implementation.
use super::style_api::finite_range;
use martensite_tkernel::{precision, standard::Failure};
use martensite_tkmath::gp::{Point2, Point3};
use std::cell::RefCell;
fn line(t: f64) -> Result<Point3, Failure> {
    Ok(Point3::new(t, 0., 0.))
}
#[test]
fn finite_endpoints_do_not_evaluate_geometry() {
    let range = finite_range::<Point3>((1., 2.), |_| {
        panic!("finite range must not evaluate geometry")
    });
    assert_eq!(range.unwrap(), (1., 2.));
}
#[test]
fn two_infinite_ends_double_until_line_distance_reaches_four_hundred() {
    let extent = precision::infinite();
    assert_eq!(
        finite_range((-extent, extent), line).unwrap(),
        (-256., 256.)
    );
    let plane = |t| Ok::<Point2, Failure>(Point2::new(t, 0.));
    assert_eq!(
        finite_range((-extent, extent), plane).unwrap(),
        (-256., 256.)
    );
}
#[test]
fn one_infinite_end_keeps_the_finite_end_and_uses_a_power_of_two() {
    for end in [-17., 0., 23.] {
        let lower = finite_range((-precision::infinite(), end), line).unwrap();
        let upper = finite_range((end, precision::infinite()), line).unwrap();
        assert_eq!(lower, (end - 512., end));
        assert_eq!(upper, (end, end + 512.));
    }
}
#[test]
fn finite_endpoint_is_read_before_one_sided_range_expansion() {
    let calls = RefCell::new(Vec::new());
    let points = |t| {
        calls.borrow_mut().push(t);
        line(t)
    };
    finite_range((-precision::infinite(), 10.), points).unwrap();
    assert_eq!(
        *calls.borrow(),
        [10., 8., 6., 2., -6., -22., -54., -118., -246., -502.]
    );
}
#[test]
fn nan_distance_stops_the_first_range_doubling() {
    let calls = RefCell::new(Vec::new());
    let points = |t| {
        calls.borrow_mut().push(t);
        Ok::<Point3, Failure>(Point3::new(f64::NAN, 0., 0.))
    };
    let result = finite_range((-precision::infinite(), precision::infinite()), points).unwrap();
    assert_eq!(result, (-2., 2.));
    assert_eq!(*calls.borrow(), [-2., 2.]);
}
#[test]
fn failed_first_endpoint_prevents_second_endpoint_evaluation() {
    let calls = RefCell::new(Vec::new());
    let points = |t| {
        calls.borrow_mut().push(t);
        Err::<Point3, Failure>(Failure::with_message(
            "RangeEvaluationUnavailable",
            &format!("u={t}: expected defined line"),
        ))
    };
    let error = finite_range((-precision::infinite(), precision::infinite()), points).unwrap_err();
    assert_eq!(error.exception_type(), "RangeEvaluationUnavailable");
    assert_eq!(error.message(), "u=-2: expected defined line");
    assert_eq!(*calls.borrow(), [-2.]);
}
