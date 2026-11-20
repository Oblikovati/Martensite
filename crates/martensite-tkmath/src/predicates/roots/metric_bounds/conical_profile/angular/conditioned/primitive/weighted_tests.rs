// SPDX-License-Identifier: GPL-2.0-only
//! The shared primitive accepts actual denominator certificates without geometry substitution.
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
fn shared_constant_denominator_matches_independent_endpoint_spike_integral() {
    execute(|arithmetic| {
        let residual = capture(&[4., 0., 0., 0.], arithmetic);
        let denominator = arithmetic.scalar(2.).unwrap();
        let result = weighted_primitive_error(
            &residual,
            &arithmetic.zero(),
            &denominator,
            &arithmetic.zero(),
            arithmetic,
        )
        .unwrap();
        assert_eq!(upper(&result, arithmetic), 0.5);
    });
}
fn logarithmic_rate(
    coefficients: &[Interval],
    minimum: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Interval {
    let rate = derivative(coefficients, arithmetic).unwrap();
    arithmetic
        .divide(&magnitude_interval(&rate), minimum)
        .unwrap()
}
#[test]
fn actual_varying_denominator_rate_prevents_the_omitted_remainder_failure() {
    execute(check_denominator_remainder);
}
fn check_denominator_remainder(arithmetic: &mut Arithmetic<'_>) {
    let residual = capture(&[-4., -2., 0., 2., 4., 6.], arithmetic);
    let coefficients = capture(&[4., 2., 1.], arithmetic);
    let minimum = arithmetic.scalar(1.).unwrap();
    let rate = logarithmic_rate(&coefficients, &minimum, arithmetic);
    let omitted = weighted_primitive_error(
        &residual,
        &arithmetic.zero(),
        &minimum,
        &arithmetic.zero(),
        arithmetic,
    )
    .unwrap();
    let complete =
        weighted_primitive_error(&residual, &arithmetic.zero(), &minimum, &rate, arithmetic)
            .unwrap();
    let actual = 8. - 10. * martensite_cmath::log(2.);
    assert!(upper(&omitted, arithmetic) < actual);
    assert!(upper(&complete, arithmetic) >= actual);
}
#[test]
fn actual_half_angle_polynomial_crossing_zero_still_has_denominator_at_least_one() {
    execute(check_crossing_denominator);
}
fn check_crossing_denominator(arithmetic: &mut Arithmetic<'_>) {
    let p = capture(&[-1., 1.], arithmetic);
    let squared = product(&p, &p, arithmetic).unwrap();
    let one = arithmetic.scalar(1.).unwrap();
    let coefficients: Vec<_> = squared
        .iter()
        .map(|v| arithmetic.add(v, &one).unwrap())
        .collect();
    assert_eq!(lower(&coefficients), 0.into());
    let minimum = Interval {
        lower: one.lower.clone().max(lower(&coefficients)),
        upper: one.upper.clone().max(lower(&coefficients)),
    };
    let rate = logarithmic_rate(&coefficients, &minimum, arithmetic);
    let residual = capture(&[1., -1.], arithmetic);
    let result =
        weighted_primitive_error(&residual, &arithmetic.zero(), &minimum, &rate, arithmetic)
            .unwrap();
    assert!(upper(&result, arithmetic) >= martensite_cmath::log(2.) / 4.);
}
fn inputs(arithmetic: &mut Arithmetic<'_>) -> (Vec<Interval>, Interval, Interval) {
    (
        capture(&[-4., -2., 0., 2., 4., 6.], arithmetic),
        arithmetic.scalar(1.).unwrap(),
        arithmetic.scalar(4.).unwrap(),
    )
}
fn run(stop: usize, limit: u32) -> (Checked<Interval>, u32, usize) {
    let (residual, minimum, rate) = execute(inputs);
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
    let result = weighted_primitive_error(
        &residual,
        &arithmetic.zero(),
        &minimum,
        &rate,
        &mut arithmetic,
    );
    let operations = arithmetic.operations();
    drop(arithmetic);
    (result, operations, polls)
}
#[test]
fn shared_primitive_honors_real_work_limits_and_initial_middle_final_cancellation() {
    let (complete, operations, polls) = run(usize::MAX, 100_000);
    assert!(complete.is_ok());
    for limit in [0, operations - 1] {
        let (result, actual, _) = run(usize::MAX, limit);
        assert!(matches!(
            result,
            Err(BoundedArithmeticFailure::WorkLimit { .. })
        ));
        assert_eq!(actual, limit);
    }
    for stop in [1, polls / 2, polls] {
        assert!(matches!(
            run(stop, 100_000).0,
            Err(BoundedArithmeticFailure::Cancelled)
        ));
    }
}
