//! Truncation float domain tests behavior and supporting operations.
use super::*;

fn linear_tail(coefficient: f64, precision: f64) -> (i32, [f64; 2]) {
    let mut bounds = [7.];
    let (mut error, mut retained) = (7., -7);
    assert_eq!(
        mmtrpjj_(
            3,
            1,
            2,
            precision,
            -1,
            &[7., coefficient, 7.],
            &mut bounds,
            &mut error,
            &mut retained
        ),
        0
    );
    (retained, [bounds[0], error])
}
#[test]
fn strict_threshold_preserves_equality_neighbors_and_signed_zero() {
    let basis = 1.5_f64.sqrt();
    for (precision, retained, error) in [
        (basis, 1, basis),
        (basis.next_down(), 2, 0.),
        (basis.next_up(), 1, basis),
        (f64::INFINITY, 1, basis),
        (f64::NEG_INFINITY, 2, 0.),
        (f64::NAN, 1, basis),
    ] {
        let result = linear_tail(1., precision);
        assert_eq!(result.0, retained);
        assert_eq!(result.1.map(f64::to_bits), [basis, error].map(f64::to_bits));
    }
    let result = linear_tail(-0., -0.);
    assert_eq!(result.0, 1);
    assert_eq!(result.1.map(f64::to_bits), [0_u64; 2]);
}
#[test]
fn nonfinite_coefficients_preserve_comparison_and_norm_categories() {
    for coefficient in [f64::INFINITY, f64::NEG_INFINITY] {
        let (retained, [bound, error]) = linear_tail(coefficient, 1.5_f64.sqrt());
        assert_eq!(retained, 2);
        assert_eq!(bound, f64::INFINITY);
        assert_eq!(error.to_bits(), 0);
    }
    let (retained, values) = linear_tail(f64::NAN, 1.5_f64.sqrt());
    assert_eq!(retained, 1);
    assert!(values.into_iter().all(f64::is_nan));
}
#[test]
fn scaled_norm_retains_retained_infinity_over_infinity_nan() {
    let mut bounds = [7.; 2];
    let (mut error, mut retained) = (7., -7);
    assert_eq!(
        mmtrpjj_(
            3,
            2,
            2,
            1.5_f64.sqrt(),
            -1,
            &[7., f64::INFINITY, 7., 7., f64::INFINITY, 7.],
            &mut bounds,
            &mut error,
            &mut retained
        ),
        0
    );
    assert_eq!(retained, 1);
    assert_eq!(bounds, [f64::INFINITY; 2]);
    assert!(error.is_nan());
}
#[test]
fn constraint_significance_is_strict_and_other_orders_dispatch_to_six() {
    for (coefficient, expected) in [(1e-9_f64, 1), (1e-9_f64.next_up(), 2)] {
        let (mut bound, mut error, mut retained) = ([7.], 7., -7);
        assert_eq!(
            mmtrpjj_(
                3,
                1,
                2,
                0.,
                0,
                &[7., coefficient, 7.],
                &mut bound,
                &mut error,
                &mut retained
            ),
            0
        );
        assert_eq!(
            (retained, bound[0].to_bits(), error.to_bits()),
            (expected, 0, 0)
        );
    }
    let (mut bound, mut error, mut retained) = ([7.], 7., -7);
    assert_eq!(
        mmtrpjj_(
            3,
            1,
            6,
            0.,
            3,
            &[7., 1., 0., 0., 0., 0.],
            &mut bound,
            &mut error,
            &mut retained
        ),
        0
    );
    assert_eq!((retained, bound[0].to_bits(), error.to_bits()), (2, 0, 0));
}
