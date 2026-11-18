// SPDX-License-Identifier: GPL-2.0-only
//! The full coefficient hull and fixed V1 subdivision independently enclose 2t(1-t).
use super::*;
use crate::predicates::roots::metric_bounds::BoundedArithmeticFailure;
use std::sync::atomic::{AtomicU32, Ordering};
fn fixed(value: i64, denominator_power: u8) -> Interval {
    let point = num_bigint::BigInt::from(value) << usize::from(128 - denominator_power);
    Interval {
        lower: point.clone(),
        upper: point,
    }
}
fn evaluate(cutoff: Option<Interval>, maximum: u32, stop: bool) -> (Checked<f64>, u32, u32) {
    let polls = AtomicU32::new(0);
    let mut cancelled = || {
        polls.fetch_add(1, Ordering::Relaxed);
        stop
    };
    let policy = ArithmeticPolicy {
        max_operations: maximum,
        ..Default::default()
    };
    let mut arithmetic = Arithmetic::new(policy, &mut cancelled);
    let poles = [fixed(0, 0), fixed(1, 0), fixed(0, 0)];
    let result = requested_magnitude(&poles, cutoff.as_ref(), &mut arithmetic)
        .and_then(|bound| Ok(arithmetic.upper_f64(bound.upper)?));
    (
        result,
        arithmetic.operations(),
        polls.load(Ordering::Relaxed),
    )
}
#[test]
fn full_domain_hull_returns_its_genuine_bound_without_clipping() {
    let (bound, operations, polls) = evaluate(Some(fixed(2, 0)), 100, false);
    let upper = bound.unwrap();
    assert_eq!(upper, 1.);
    assert_eq!((operations, polls), (0, 1));
    // The independent closed-form maximum is 1/2; the unrefined hull is conservative.
    assert!(upper >= 0.5);
}
#[test]
fn insufficient_hull_uses_unchanged_v1_refinement_instead_of_accepting_cutoff() {
    let (legacy, legacy_work, legacy_polls) = evaluate(None, 100, false);
    let (conditioned, work, polls) = evaluate(Some(fixed(1, 3)), 100, false);
    assert_eq!(legacy.unwrap().to_bits(), 0.5_f64.to_bits());
    assert_eq!(conditioned.unwrap().to_bits(), 0.5_f64.to_bits());
    assert_eq!((legacy_work, legacy_polls), (18, 18));
    assert_eq!((work, polls), (18, 19));
}
#[test]
fn legacy_work_failure_and_poll_sequence_are_preserved() {
    let (legacy, work, polls) = evaluate(None, 1, false);
    assert_eq!((work, polls), (1, 2));
    assert_work_limit(legacy.unwrap_err());
    let (conditioned, work, polls) = evaluate(Some(fixed(1, 3)), 1, false);
    assert_eq!((work, polls), (1, 3));
    assert_work_limit(conditioned.unwrap_err());
}
fn assert_work_limit(cause: CylindricalProfileFailure) {
    let CylindricalProfileFailure::Arithmetic(cause) = cause else {
        panic!("expected original arithmetic refusal");
    };
    assert!(matches!(
        *cause,
        BoundedArithmeticFailure::WorkLimit {
            completed: 1,
            limit: 1
        }
    ));
}
#[test]
fn both_conditioned_and_legacy_paths_observe_cancellation() {
    for cutoff in [None, Some(fixed(2, 0))] {
        let (result, work, polls) = evaluate(cutoff, 100, true);
        let CylindricalProfileFailure::Arithmetic(cause) = result.unwrap_err() else {
            panic!("expected cancellation");
        };
        assert!(matches!(*cause, BoundedArithmeticFailure::Cancelled));
        assert_eq!((work, polls), (0, 1));
    }
}

#[test]
fn legacy_xy_wrapper_keeps_exact_v1_bounds_operations_and_polls() {
    let polls = AtomicU32::new(0);
    let mut cancelled = || {
        polls.fetch_add(1, Ordering::Relaxed);
        false
    };
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut cancelled);
    let stored = quadratic_residual();
    let trig = [vec![fixed(0, 0)], vec![fixed(0, 0)]];
    let result = scaled_xy_residual(
        &[fixed(1, 0)],
        &stored,
        &trig,
        &fixed(1, 0),
        &mut arithmetic,
    )
    .unwrap();
    assert_eq!(arithmetic.upper_f64(result.upper).unwrap(), 0.5);
    assert_eq!(arithmetic.operations(), 58);
    assert_eq!(polls.load(Ordering::Relaxed), 58);
}

fn quadratic_residual() -> Homogeneous {
    [
        vec![fixed(0, 0), fixed(1, 0), fixed(0, 0)],
        vec![fixed(0, 0)],
        vec![fixed(0, 0)],
        vec![fixed(1, 0)],
    ]
}
