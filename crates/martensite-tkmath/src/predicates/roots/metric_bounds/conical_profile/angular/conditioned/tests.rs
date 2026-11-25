// SPDX-License-Identifier: GPL-2.0-only
//! Exact polynomial integral oracles distinguish uniform value and derivative bounds.
use super::*;
fn execute<T>(body: impl FnOnce(&mut Arithmetic<'_>) -> T) -> T {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut never);
    body(&mut arithmetic)
}
fn capture(values: &[f64], arithmetic: &mut Arithmetic<'_>) -> Vec<Interval> {
    values
        .iter()
        .map(|v| arithmetic.scalar(*v).unwrap())
        .collect()
}
fn upper(value: &Interval, arithmetic: &Arithmetic<'_>) -> f64 {
    arithmetic.upper_f64(value.upper.clone()).unwrap()
}
#[test]
fn endpoint_spike_has_exact_half_integral_and_distinct_pointwise_derivative() {
    execute(|arithmetic| {
        let residual = capture(&[4., 0., 0., 0.], arithmetic);
        let denominator = arithmetic.scalar(2.).unwrap();
        let proof = from_residual(&residual, arithmetic.zero(), &denominator, arithmetic).unwrap();
        // Integral of 4(1-t)^3/2 on [0,1] is 1/2; its maximum derivative is 2.
        assert_eq!(upper(&proof.bound, arithmetic), 0.5);
        assert_eq!(upper(&proof.derivative, arithmetic), 2.);
    });
}
#[test]
fn constant_residual_retains_exact_integral_plus_actual_anchor() {
    execute(|arithmetic| {
        let residual = capture(&[3.; 4], arithmetic);
        let denominator = arithmetic.scalar(2.).unwrap();
        let anchor = arithmetic.scalar(0.25).unwrap();
        let proof = from_residual(&residual, anchor, &denominator, arithmetic).unwrap();
        assert_eq!(upper(&proof.bound, arithmetic), 1.75);
        assert_eq!(upper(&proof.derivative, arithmetic), 1.5);
    });
}
#[test]
fn mixed_sign_residual_bounds_every_partial_integral_without_cancelling_its_derivative() {
    execute(|arithmetic| {
        let residual = capture(&[3., 0., 0., -3.], arithmetic);
        let denominator = arithmetic.scalar(1.).unwrap();
        let proof = from_residual(&residual, arithmetic.zero(), &denominator, arithmetic).unwrap();
        // The signed integral peaks at t=1/2 with exact value 21/32 and ends at zero.
        assert!(upper(&proof.bound, arithmetic) >= 21. / 32.);
        assert_eq!(upper(&proof.bound, arithmetic), 1.5);
        assert_eq!(upper(&proof.derivative, arithmetic), 3.);
    });
}
#[test]
fn fit_hull_crossing_zero_retains_base_and_negative_hull_uses_positive_distance() {
    execute(|arithmetic| {
        let q = capture(&[4., 4.], arithmetic);
        let sine = arithmetic.scalar(0.5).unwrap();
        let crossing = capture(&[-1., 1.], arithmetic);
        let negative = capture(&[-3., -2.], arithmetic);
        let positive = capture(&[2., 3.], arithmetic);
        assert_eq!(
            upper(
                &full_denominator(&q, &sine, &crossing, arithmetic).unwrap(),
                arithmetic
            ),
            2.
        );
        let first = full_denominator(&q, &sine, &negative, arithmetic).unwrap();
        let second = full_denominator(&q, &sine, &positive, arithmetic).unwrap();
        assert_eq!(first, second);
        assert_eq!(upper(&first, arithmetic), 10.);
    });
}
#[test]
fn unresolved_denominator_never_publishes_an_integral_bound() {
    execute(|arithmetic| {
        let residual = capture(&[1., 1.], arithmetic);
        for value in [0., -1.] {
            let denominator = arithmetic.scalar(value).unwrap();
            assert!(matches!(
                from_residual(&residual, arithmetic.zero(), &denominator, arithmetic),
                Err(AngularSpanFailure::Arithmetic(
                    BoundedArithmeticFailure::DenominatorUnresolved { .. }
                ))
            ));
        }
    });
}
fn run(stop: usize, limit: u32) -> (Result<AngularEvidence, AngularSpanFailure>, u32, usize) {
    let (residual, denominator) = execute(|arithmetic| {
        (
            capture(&[4., 0., 0., 0.], arithmetic),
            arithmetic.scalar(2.).unwrap(),
        )
    });
    let mut polls = 0;
    let mut callback = || {
        polls += 1;
        polls == stop
    };
    let policy = ArithmeticPolicy {
        max_operations: limit,
        ..Default::default()
    };
    let mut arithmetic = Arithmetic::new(policy, &mut callback);
    let result = from_residual(&residual, arithmetic.zero(), &denominator, &mut arithmetic);
    let operations = arithmetic.operations();
    drop(arithmetic);
    (result, operations, polls)
}
#[test]
fn actual_work_limits_include_all_mean_and_denominator_computation() {
    let (proof, complete, _) = run(usize::MAX, 100_000);
    assert!(proof.is_ok());
    for limit in [0, complete - 1] {
        let (result, actual, _) = run(usize::MAX, limit);
        assert!(matches!(
            result,
            Err(AngularSpanFailure::Arithmetic(
                BoundedArithmeticFailure::WorkLimit { .. }
            ))
        ));
        assert_eq!(actual, limit);
    }
}
#[test]
fn initial_middle_and_final_cancellation_return_no_partial_bound() {
    let (_, _, polls) = run(usize::MAX, 100_000);
    for stop in [1, polls / 2, polls] {
        assert!(matches!(
            run(stop, 100_000).0,
            Err(AngularSpanFailure::Arithmetic(
                BoundedArithmeticFailure::Cancelled
            ))
        ));
    }
}
#[test]
fn legacy_angular_result_work_and_polls_remain_isolated() {
    let before = legacy();
    run(usize::MAX, 100_000).0.unwrap();
    assert_eq!(before, legacy());
}
fn legacy() -> (Interval, Interval, u32, usize) {
    let mut polls = 0;
    let mut callback = || {
        polls += 1;
        false
    };
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut callback);
    let source = admit_source(&[[80., 0.], [80., 1.]], &mut arithmetic).unwrap();
    let q = squared_distance(&source, &mut arithmetic).unwrap();
    let fit = capture(&[0., 0.05], &mut arithmetic);
    let sine = arithmetic.scalar(0.125).unwrap();
    let proof = angular_from_captured(&source, &fit, &q, arithmetic.zero(), &sine, &mut arithmetic)
        .unwrap();
    let operations = arithmetic.operations();
    drop(arithmetic);
    (proof.bound, proof.derivative, operations, polls)
}
