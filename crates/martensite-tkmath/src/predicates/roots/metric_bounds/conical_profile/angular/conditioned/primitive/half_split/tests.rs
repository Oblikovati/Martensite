// SPDX-License-Identifier: GPL-2.0-only
//! Independent closed forms and actual bounded-work/cancellation controls for explicit refinement.
use super::*;
fn execute<T>(body: impl FnOnce(&mut Arithmetic<'_>) -> T) -> T {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut never);
    body(&mut arithmetic)
}
fn capture(values: &[f64], arithmetic: &mut Arithmetic<'_>) -> Vec<Interval> {
    values
        .iter()
        .map(|value| arithmetic.scalar(*value).unwrap())
        .collect()
}
fn upper(value: Interval, arithmetic: &Arithmetic<'_>) -> f64 {
    arithmetic.upper_f64(value.upper).unwrap()
}
#[test]
fn signed_quadratic_primitive_has_independent_half_maximum_and_third_integral() {
    execute(|arithmetic| {
        // P(t)=2t(1-t), so max|P|=1/2 and integral|P|=1/3.
        let primitive = capture(&[0., 1., 0.], arithmetic);
        let old = absolute_coefficient_mean(&primitive, arithmetic).unwrap();
        let (mean, maximum) = metrics(&primitive, &old, arithmetic).unwrap();
        assert_eq!(upper(maximum, arithmetic), 0.5);
        assert!(upper(mean, arithmetic) >= 1. / 3.);
        assert!(upper(old, arithmetic) >= 1. / 3.);
    });
}
#[test]
fn constant_and_zero_primitives_preserve_the_independent_weighted_value() {
    execute(|arithmetic| {
        check_constant(3., 2., 2., 3.5, arithmetic);
        check_constant(0., 0., 1., 0., arithmetic);
    });
}
fn check_constant(
    value: f64,
    anchor: f64,
    denominator: f64,
    expected: f64,
    arithmetic: &mut Arithmetic<'_>,
) {
    let primitive = capture(&[value; 3], arithmetic);
    let anchor = arithmetic.scalar(anchor).unwrap();
    let denominator = arithmetic.scalar(denominator).unwrap();
    let bound = weighted_from_primitive(
        &primitive,
        &anchor,
        &denominator,
        &arithmetic.zero(),
        arithmetic,
    )
    .unwrap();
    assert_eq!(upper(bound, arithmetic), expected);
}
fn run(limit: u32, stop: usize) -> (Checked<Interval>, u32, usize) {
    let primitive = execute(|arithmetic| capture(&[0., 1., 0.], arithmetic));
    let denominator = execute(|arithmetic| arithmetic.scalar(1.).unwrap());
    let mut polls = 0;
    let mut cancelled = || {
        polls += 1;
        polls == stop
    };
    let policy = policy(limit);
    let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
    let result = weighted_from_primitive_selected(
        &primitive,
        &arithmetic.zero(),
        &denominator,
        &arithmetic.zero(),
        ConicalPrimitiveRefinement::HalfSplitV1,
        &mut arithmetic,
    );
    (result, arithmetic.operations(), polls)
}
#[test]
fn every_added_split_operation_uses_the_original_limit() {
    let (result, complete, _) = run(1000, usize::MAX);
    assert!(result.is_ok());
    for limit in [0, 1, complete - 1] {
        let (result, work, _) = run(limit, usize::MAX);
        assert!(
            matches!(result, Err(BoundedArithmeticFailure::WorkLimit { completed, limit: declared }) if completed == limit && declared == limit)
        );
        assert_eq!(work, limit);
    }
}
#[test]
fn cancellation_at_first_and_final_actual_split_checkpoints_returns_no_bound() {
    let (_, _, polls) = run(1000, usize::MAX);
    for stop in [1, polls] {
        let (result, work, observed) = run(1000, stop);
        assert!(matches!(result, Err(BoundedArithmeticFailure::Cancelled)));
        assert_eq!(observed, stop);
        assert_eq!(work as usize, stop - 1);
    }
}
#[test]
fn sign_changing_primitive_retains_independent_quarter_absolute_integral() {
    execute(|arithmetic| {
        // P(t)=t(2t-1): its negative and positive integrals have magnitudes 1/24 and 5/24.
        let primitive = capture(&[0., -0.5, 1.], arithmetic);
        let old = absolute_coefficient_mean(&primitive, arithmetic).unwrap();
        let (mean, maximum) = metrics(&primitive, &old, arithmetic).unwrap();
        assert_eq!(upper(maximum, arithmetic), 1.);
        let mean = upper(mean, arithmetic);
        assert!(mean >= 0.25);
        assert!(mean < upper(old, arithmetic));
    });
}

fn policy(limit: u32) -> ArithmeticPolicy {
    ArithmeticPolicy {
        max_operations: limit,
        ..ArithmeticPolicy::default()
    }
}
#[test]
fn explicit_unsplit_preserves_work_and_split_charges_each_new_arithmetic_call() {
    execute(|arithmetic| {
        let primitive = capture(&[0., 1., 0.], arithmetic);
        let one = arithmetic.scalar(1.).unwrap();
        let (old, old_work) = measured(&primitive, &one, None, arithmetic);
        let (same, same_work) = measured(
            &primitive,
            &one,
            Some(ConicalPrimitiveRefinement::UnsplitV1),
            arithmetic,
        );
        assert_eq!((old.upper, old_work), (same.upper, same_work));
        let (selected, selected_work) = measured(
            &primitive,
            &one,
            Some(ConicalPrimitiveRefinement::HalfSplitV1),
            arithmetic,
        );
        assert_eq!(upper(selected, arithmetic), 0.5);
        assert_eq!(selected_work, old_work + 14);
    });
}
fn measured(
    primitive: &[Interval],
    one: &Interval,
    mode: Option<ConicalPrimitiveRefinement>,
    arithmetic: &mut Arithmetic<'_>,
) -> (Interval, u32) {
    let zero = arithmetic.zero();
    let start = arithmetic.operations();
    let result = match mode {
        None => weighted_from_primitive(primitive, &zero, one, &zero, arithmetic),
        Some(mode) => {
            weighted_from_primitive_selected(primitive, &zero, one, &zero, mode, arithmetic)
        }
    }
    .unwrap();
    (result, arithmetic.operations() - start)
}
