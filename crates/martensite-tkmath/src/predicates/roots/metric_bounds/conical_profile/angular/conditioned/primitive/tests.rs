// SPDX-License-Identifier: GPL-2.0-only
//! Independent exact primitives and a denominator-variation counterexample.
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
fn exact_constant_and_endpoint_spike_primitives_retain_their_endpoint_integrals() {
    execute(|arithmetic| {
        let constant = primitive(&capture(&[3.; 4], arithmetic), arithmetic).unwrap();
        assert_eq!(constant.len(), 5);
        assert_eq!(upper(constant.last().unwrap(), arithmetic), 3.);
        let spike = primitive(&capture(&[4., 0., 0., 0.], arithmetic), arithmetic).unwrap();
        assert_eq!(upper(spike.last().unwrap(), arithmetic), 1.);
    });
}
#[test]
fn mixed_sign_primitive_exploits_exact_cancellation_with_constant_denominator() {
    execute(check_constant_denominator);
}
fn check_constant_denominator(arithmetic: &mut Arithmetic<'_>) {
    let residual = capture(&[-4., -2., 0., 2., 4., 6.], arithmetic);
    let q = capture(&[1.], arithmetic);
    let fit = capture(&[0.], arithmetic);
    let denominator = arithmetic.scalar(1.).unwrap();
    let proof = bound(
        &residual,
        &q,
        &fit,
        &denominator,
        &arithmetic.zero(),
        arithmetic,
    )
    .unwrap();
    // P(t)=5t²-4t; its degree-six coefficients occupy [-1,1].
    assert_eq!(upper(&proof, arithmetic), 1.);
    let old = from_residual(&residual, arithmetic.zero(), &denominator, arithmetic).unwrap();
    assert_eq!(upper(&old.bound, arithmetic), 3.);
    assert_eq!(upper(&old.derivative, arithmetic), 6.);
}
#[test]
fn varying_denominator_requires_the_integration_by_parts_remainder() {
    execute(check_varying_denominator);
}
fn check_varying_denominator(arithmetic: &mut Arithmetic<'_>) {
    let residual = capture(&[-4., -2., 0., 2., 4., 6.], arithmetic);
    let q = capture(&[4., 2., 1.], arithmetic);
    let fit = capture(&[0.], arithmetic);
    let denominator = arithmetic.scalar(1.).unwrap();
    let p = primitive(&residual, arithmetic).unwrap();
    let omitted = arithmetic
        .divide(&magnitude_interval(&p), &denominator)
        .unwrap();
    // Integral (10t-4)/(2-t)² is 8-10ln2 > 21/20, using ln2<139/200.
    assert!(upper(&omitted, arithmetic) < 21. / 20.);
    let result = bound_for(&residual, &q, &fit, &denominator, arithmetic);
    let actual_integral = 8. - 10. * martensite_cmath::log(2.);
    assert!(actual_integral > 21. / 20.);
    assert!(upper(&result, arithmetic) >= actual_integral);
    assert!(upper(&result, arithmetic) > upper(&omitted, arithmetic));
}
#[test]
fn logarithmic_denominator_rate_bounds_both_radius_and_signed_fit_ranges() {
    execute(|arithmetic| {
        let q = capture(&[4., 2., 1.], arithmetic);
        let zero = capture(&[0.], arithmetic);
        let rate = denominator_rate(&q, &zero, arithmetic).unwrap();
        assert_eq!(upper(&rate, arithmetic), 4.);
        let positive = capture(&[2., 3.], arithmetic);
        let negative = capture(&[-2., -3.], arithmetic);
        let first = fit_rate(&positive, arithmetic).unwrap();
        let second = fit_rate(&negative, arithmetic).unwrap();
        assert_eq!(first, second);
        assert!(upper(&first, arithmetic) >= 1.2);
        let crossing = capture(&[-1., 1.], arithmetic);
        assert_eq!(
            upper(&fit_rate(&crossing, arithmetic).unwrap(), arithmetic),
            4.
        );
    });
}
fn run(stop: usize, limit: u32) -> (Result<Interval, AngularSpanFailure>, u32, usize) {
    let (residual, q, fit, denominator) = execute(inputs);
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
    let result = attempt_bound(&residual, &q, &fit, &denominator, &mut arithmetic);
    let work = arithmetic.operations();
    drop(arithmetic);
    (result, work, polls)
}
fn attempt_bound(
    residual: &[Interval],
    q: &[Interval],
    fit: &[Interval],
    denominator: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Interval, AngularSpanFailure> {
    bound(
        residual,
        q,
        fit,
        denominator,
        &arithmetic.zero(),
        arithmetic,
    )
}
#[test]
fn actual_primitive_work_and_cancellation_never_publish_partial_bounds() {
    let (complete, work, polls) = run(usize::MAX, 100_000);
    assert!(complete.is_ok());
    assert_work_limits(work);
    for stop in [1, polls / 2, polls] {
        assert!(matches!(
            run(stop, 100_000).0,
            Err(AngularSpanFailure::Arithmetic(
                BoundedArithmeticFailure::Cancelled
            ))
        ));
    }
}

fn inputs(
    arithmetic: &mut Arithmetic<'_>,
) -> (Vec<Interval>, Vec<Interval>, Vec<Interval>, Interval) {
    (
        capture(&[-4., -2., 0., 2., 4., 6.], arithmetic),
        capture(&[4., 2., 1.], arithmetic),
        capture(&[0.], arithmetic),
        arithmetic.scalar(1.).unwrap(),
    )
}
fn bound_for(
    residual: &[Interval],
    q: &[Interval],
    fit: &[Interval],
    denominator: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Interval {
    attempt_bound(residual, q, fit, denominator, arithmetic).unwrap()
}

fn assert_work_limits(work: u32) {
    for limit in [0, work - 1] {
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
