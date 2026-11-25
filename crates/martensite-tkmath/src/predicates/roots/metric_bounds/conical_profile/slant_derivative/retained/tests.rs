// SPDX-License-Identifier: GPL-2.0-only
use super::*;
fn execute<T>(body: impl FnOnce(&mut Arithmetic<'_>) -> T) -> T {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut never);
    body(&mut arithmetic)
}
fn capture(values: &[f64], arithmetic: &mut Arithmetic<'_>) -> Vec<Interval> {
    slant::capture(values, arithmetic).unwrap()
}
#[test]
fn exact_linear_radius_has_zero_derivative_error_for_nonunit_weight() {
    execute(|arithmetic| {
        let q = capture(&[4., 6., 9.], arithmetic);
        let n = capture(&[4., 6.], arithmetic);
        let w = capture(&[2., 2.], arithmetic);
        let proof = slant::capture_in_chart(&q, &n, &w, arithmetic).unwrap();
        let (error, rate) = retained_derivative(&q, &n, &proof, arithmetic)
            .unwrap()
            .unwrap();
        assert!(arithmetic.upper_f64(error.upper).unwrap() < 1e-25);
        assert!(rate.lower <= arithmetic.scalar(1.).unwrap().lower);
        assert!(arithmetic.upper_f64(rate.upper).unwrap() >= 1.);
    });
}
#[test]
fn constant_original_radius_and_linear_fit_cover_independent_unit_error_rate() {
    execute(|arithmetic| {
        let q = capture(&[9., 9., 9.], arithmetic);
        let n = capture(&[6., 8.], arithmetic);
        let w = capture(&[2., 2.], arithmetic);
        let proof = slant::capture_in_chart(&q, &n, &w, arithmetic).unwrap();
        let (error, rate) = retained_derivative(&q, &n, &proof, arithmetic)
            .unwrap()
            .unwrap();
        assert!(arithmetic.upper_f64(error.upper).unwrap() >= 1.);
        assert!(arithmetic.upper_f64(rate.upper).unwrap() >= 1.);
    });
}
#[test]
fn variable_weights_select_existing_general_derivative_proof() {
    execute(|arithmetic| {
        let q = capture(&[4., 6., 9.], arithmetic);
        let n = capture(&[4., 6.], arithmetic);
        let w = capture(&[1., 2.], arithmetic);
        let proof = slant::capture_in_chart(&q, &n, &w, arithmetic).unwrap();
        let before = arithmetic.operations();
        assert!(
            retained_derivative(&q, &n, &proof, arithmetic)
                .unwrap()
                .is_none()
        );
        assert_eq!(before, arithmetic.operations());
    });
}
#[test]
fn actual_retained_residual_derivative_work_is_less_than_recomputation() {
    execute(|arithmetic| {
        let q = capture(&[4., 6., 9.], arithmetic);
        let n = capture(&[4., 6.], arithmetic);
        let w = capture(&[2., 2.], arithmetic);
        let proof = slant::capture_in_chart(&q, &n, &w, arithmetic).unwrap();
        let start = arithmetic.operations();
        retained_derivative(&q, &n, &proof, arithmetic).unwrap();
        let reduced = arithmetic.operations() - start;
        let start = arithmetic.operations();
        radial_derivative_bound(&q, &n, &w, arithmetic).unwrap();
        assert!(reduced < arithmetic.operations() - start);
    });
}
#[test]
fn retained_derivative_cancellation_never_returns_a_bound() {
    let (q, n, proof) = execute(|arithmetic| {
        let q = capture(&[4., 6., 9.], arithmetic);
        let n = capture(&[4., 6.], arithmetic);
        let w = capture(&[2., 2.], arithmetic);
        let proof = slant::capture_in_chart(&q, &n, &w, arithmetic).unwrap();
        (q, n, proof)
    });
    let mut stop = || true;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut stop);
    assert!(matches!(
        retained_derivative(&q, &n, &proof, &mut arithmetic),
        Err(ConicalOriginalFailure::Arithmetic(
            BoundedArithmeticFailure::Cancelled
        ))
    ));
}
