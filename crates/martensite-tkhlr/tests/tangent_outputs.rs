//! Caller-owned endpoint output effects, pinned before exposing the new entry point.
use martensite_tkmath::gp::{Direction2, Point2};
#[path = "tangent_outputs/fixture.rs"]
mod fixture;
const NATIVE: &str = include_str!("fixtures/tangent_outputs/reference-observations.txt");

#[test]
fn frozen_native_endpoint_outputs_and_diagnostics_are_retained() {
    assert_eq!(NATIVE.lines().count(), 30);
    for observation in NATIVE.lines() {
        let fields: Vec<&str> = observation.split('|').collect();
        let curve = fixture::projected_fixture(fields[0]);
        let (mut point, mut direction) = (Point2::new(7., 8.), Direction2::Y);
        let result = curve.tangent_into(fields[1] == "1", &mut point, &mut direction);
        assert_outcome(result, fields[3], fields[4]);
        assert_bits(fields[6], [point.x(), point.y()]);
        assert_bits(fields[7], [direction.x(), direction.y()]);
    }
}

fn assert_outcome(
    result: Result<(), martensite_tkernel::standard::Failure>,
    kind: &str,
    message: &str,
) {
    match result {
        Ok(()) => assert_eq!(kind, "OK"),
        Err(failure) => {
            assert_eq!(failure.exception_type(), kind);
            assert_eq!(failure.message_bytes(), message.as_bytes());
        }
    }
}

fn assert_bits(expected: &str, actual: [f64; 2]) {
    let bits: Vec<u64> = expected
        .split(',')
        .map(|field| field.parse().unwrap())
        .collect();
    assert_eq!(actual.map(f64::to_bits), [bits[0], bits[1]]);
}

#[test]
fn failed_vertical_tangent_writes_point_and_preserves_direction() {
    let curve = fixture::projected_fixture("top_z");
    for start in [true, false] {
        let (mut point, mut direction) = (Point2::new(7., 8.), Direction2::Y);
        assert!(
            curve
                .tangent_into(start, &mut point, &mut direction)
                .is_err()
        );
        assert_eq!(point, Point2::new(1., 2.));
        assert_eq!(direction, Direction2::Y);
    }
}

#[test]
fn linear_endpoint_outputs_follow_exact_parametric_geometry() {
    let curve = fixture::projected_fixture("top_x");
    for (start, x) in [(true, 1.), (false, 3.)] {
        let (mut point, mut direction) = (Point2::new(7., 8.), Direction2::Y);
        curve
            .tangent_into(start, &mut point, &mut direction)
            .unwrap();
        assert_eq!(point, Point2::new(x, 2.));
        assert_eq!(direction, Direction2::X);
        assert_eq!(curve.tangent(start).unwrap(), (point, direction));
    }
}

#[test]
fn stationary_endpoint_uses_nonzero_second_derivative() {
    let curve = fixture::projected_fixture("stationary_start");
    // The projected Bezier is (u², 0), whose second derivative at zero is (2, 0).
    let (mut point, mut direction) = (Point2::new(7., 8.), Direction2::Y);
    curve
        .tangent_into(true, &mut point, &mut direction)
        .unwrap();
    assert_eq!(point, Point2::default());
    assert_eq!(direction, Direction2::X);
}

#[test]
fn special_projection_preserves_caller_point_on_success_and_failure() {
    for label in ["type0_x", "type0_normal"] {
        let curve = fixture::projected_fixture(label);
        let (mut point, mut direction) = (Point2::new(7., 8.), Direction2::Y);
        let result = curve.tangent_into(true, &mut point, &mut direction);
        assert_eq!(point, Point2::new(7., 8.));
        if label == "type0_normal" {
            assert!(result.is_err());
            assert_eq!(direction, Direction2::Y);
            continue;
        }
        result.unwrap();
        assert_eq!(direction.x(), direction.y());
        assert!(direction.x() > 0.);
        assert!((0.5_f64).sqrt() >= direction.x());
        assert!(direction.x().next_up() >= (0.5_f64).sqrt());
    }
}
