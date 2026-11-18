// SPDX-License-Identifier: GPL-2.0-only
use super::curve::*;
use super::*;

#[test]
fn accounted_quotient_matches_ordinary_bounds_work_and_poll_sequence() {
    let (ordinary, first_polls) = counted_quotient(false);
    let (accounted, second_polls) = counted_quotient(true);
    assert_eq!(ordinary.bounds.lower, accounted.bounds.lower);
    assert_eq!(ordinary.bounds.upper, accounted.bounds.upper);
    assert_eq!(ordinary.subintervals, accounted.subintervals);
    assert_eq!(first_polls, second_polls);
}

#[test]
fn accounted_constant_quotient_is_exact_without_examining_a_cell() {
    let options = RationalIntegralOptions::default();
    let (result, examined) = quotient_integral_accounted(
        vec![6.into()],
        vec![2.into()],
        options,
        &ExactRatio::from_finite(0.),
        &mut || false,
    );
    let exact = result.unwrap();
    assert_eq!(exact.bounds.lower, ExactRatio::from_finite(1.5));
    assert_eq!(exact.bounds.upper, exact.bounds.lower);
    assert_eq!(examined, 0);
}

#[test]
fn failed_quotient_retains_examined_cell_without_counting_a_refused_charge() {
    let options = RationalIntegralOptions::default();
    let (cancelled, examined) = run(options, &mut || true);
    assert!(cancelled.is_err());
    assert_eq!(examined, 1);
    let (exhausted, examined) = run(
        RationalIntegralOptions {
            max_subintervals: 0,
            ..options
        },
        &mut || false,
    );
    assert!(matches!(
        exhausted,
        Err(RationalIntegralFailure::WorkLimit { limit: 0, .. })
    ));
    assert_eq!(examined, 0);
}

fn run(
    options: RationalIntegralOptions,
    cancelled: &mut impl FnMut() -> bool,
) -> (Result<CurveIntegral, RationalIntegralFailure>, u64) {
    quotient_integral_accounted(
        vec![2.into()],
        vec![1.into(), 0.into(), 1.into()],
        options,
        &ExactRatio::from_finite(1e-10),
        cancelled,
    )
}

fn counted_quotient(accounted: bool) -> (CurveIntegral, usize) {
    let options = RationalIntegralOptions::default();
    let mut polls = 0;
    let mut cancelled = || {
        polls += 1;
        false
    };
    let (result, examined) = if accounted {
        run(options, &mut cancelled)
    } else {
        ordinary(options, &mut cancelled)
    };
    let report = result.unwrap();
    assert_eq!(examined, report.subintervals);
    (report, polls)
}
fn ordinary(
    options: RationalIntegralOptions,
    cancelled: &mut impl FnMut() -> bool,
) -> (Result<CurveIntegral, RationalIntegralFailure>, u64) {
    let result = quotient_integral(
        vec![2.into()],
        vec![1.into(), 0.into(), 1.into()],
        options,
        &ExactRatio::from_finite(1e-10),
        cancelled,
    );
    let examined = result.as_ref().map_or(0, |report| report.subintervals);
    (result, examined)
}
