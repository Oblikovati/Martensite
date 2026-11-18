// SPDX-License-Identifier: GPL-2.0-only
//! Closed-form controls cover full brackets, rejected roots and exact eager fallback.
use super::*;
use crate::predicates::roots::{RootIsolationOptions, isolate_polynomial_roots};
struct Fixture {
    source: IntegerPolynomial,
    query: IntegerPolynomial,
    isolation: RootIsolation,
}
fn fixture(source: &[f64], query: &[f64], bounds: [f64; 2], width: f64) -> Fixture {
    let isolation =
        isolate_polynomial_roots(source, bounds, RootIsolationOptions::new(width), || false)
            .unwrap();
    Fixture {
        source: IntegerPolynomial::from_binary64(source, 32).unwrap(),
        query: IntegerPolynomial::accept_zero(IntegerPolynomial::from_binary64(query, 32)).unwrap(),
        isolation,
    }
}
fn run(
    f: &Fixture,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<RootSignEvaluation, RootSignError> {
    evaluate(
        f.source.clone(),
        f.query.clone(),
        f.isolation.clone(),
        cancelled,
    )
}
fn attempted(f: &Fixture) -> (Option<Vec<Ordering>>, usize) {
    let mut calls = 0;
    let result = attempt(&f.query, &f.isolation, &mut || {
        calls += 1;
        false
    })
    .unwrap();
    (result, calls)
}
fn eager(f: &Fixture) -> (RootSignEvaluation, usize) {
    let mut calls = 0;
    let result = evaluate_isolated(
        f.source.clone(),
        f.query.clone(),
        f.isolation.clone(),
        &mut || {
            calls += 1;
            false
        },
    )
    .unwrap();
    (result, calls)
}
fn assert_result(f: &Fixture, expected: &[Ordering]) {
    let result = run(f, &mut || false).unwrap();
    assert_eq!(result.isolation(), &f.isolation);
    assert_eq!(result.signs(), expected);
    assert_eq!(result, eager(f).0);
}
#[test]
fn strict_full_brackets_certify_both_signs_without_refinement() {
    let f = fixture(&[-2., 0., 1.], &[0., 1.], [-2., 2.], 0.001);
    let expected = vec![Ordering::Less, Ordering::Greater];
    assert_eq!(attempted(&f).0, Some(expected.clone()));
    assert_result(&f, &expected);
}
#[test]
fn exact_singleton_zero_remains_decisive_among_nonpoint_roots() {
    let f = fixture(&[0., -2., 0., 1.], &[0., 1.], [0., 2.], 0.001);
    let expected = vec![Ordering::Equal, Ordering::Greater];
    assert_eq!(attempted(&f).0, Some(expected.clone()));
    assert_result(&f, &expected);
}
#[test]
fn only_empty_normalized_query_is_globally_zero() {
    let zero = fixture(&[-2., 0., 1.], &[0., 0., 0.], [-2., 2.], 0.001);
    assert!(zero.query.0.is_empty());
    assert_eq!(attempted(&zero).0, Some(vec![Ordering::Equal; 2]));
    assert_result(&zero, &[Ordering::Equal; 2]);
    let common = fixture(&[-2., 0., 1.], &[-2., 0., 1.], [-2., 2.], 0.001);
    assert!(!common.query.0.is_empty());
    assert_fallback(&common, &[Ordering::Equal; 2]);
}
#[test]
fn midpoint_sign_is_not_an_algebraic_root_sign() {
    // sqrt(2) lies in [1,2], below 29/20; its bracket midpoint is above 29/20.
    let f = fixture(&[-2., 0., 1.], &[-29., 20.], [1., 2.], 1.);
    assert_eq!(
        f.query
            .sign_at(&super::super::super::DyadicBound::from_f64(1.5).unwrap()),
        num_bigint::Sign::Plus
    );
    assert_fallback(&f, &[Ordering::Less]);
}
#[test]
fn enclosure_touching_zero_falls_back_despite_positive_root_sign() {
    let f = fixture(&[-2., 0., 1.], &[-1., 1.], [1., 2.], 1.);
    assert_fallback(&f, &[Ordering::Greater]);
}
fn assert_fallback(f: &Fixture, expected: &[Ordering]) {
    let (attempt, extra) = attempted(f);
    assert!(attempt.is_none());
    let mut calls = 0;
    let result = run(f, &mut || {
        calls += 1;
        false
    })
    .unwrap();
    let old = eager(f);
    assert_eq!(result, old.0);
    assert_eq!(calls, extra + old.1);
    assert_result(f, expected);
}
#[test]
fn repeated_roots_keep_their_multiplicity_and_original_isolation() {
    let f = fixture(&[4., 0., -4., 0., 1.], &[0., 1.], [-2., 2.], 0.001);
    assert_result(&f, &[Ordering::Less, Ordering::Greater]);
    assert!(f.isolation.roots().iter().all(|r| r.multiplicity() == 2));
}
#[test]
fn one_unknown_discards_all_provisional_signs_and_evaluates_every_root() {
    // First root is strictly below 7/5; the coarse positive bracket contains 7/5.
    let f = fixture(&[-2., 0., 1.], &[-7., 5.], [-2., 2.], 1.);
    assert_fallback(&f, &[Ordering::Less, Ordering::Greater]);
}
#[test]
fn every_filter_and_fallback_checkpoint_cancels_as_original_evaluation_error() {
    for f in [
        fixture(&[-2., 0., 1.], &[0., 1.], [-2., 2.], 0.001),
        fixture(&[-2., 0., 1.], &[-2., 0., 1.], [-2., 2.], 0.001),
        fixture(&[0., -2., 0., 1.], &[0., 1.], [0., 2.], 0.001),
        fixture(&[-2., 0., 1.], &[], [-2., 2.], 0.001),
    ] {
        assert_all_cancellations(&f);
    }
}
fn assert_all_cancellations(f: &Fixture) {
    let mut checkpoints = 0;
    run(f, &mut || {
        checkpoints += 1;
        false
    })
    .unwrap();
    for stop in 1..=checkpoints {
        let mut calls = 0;
        let result = run(f, &mut || {
            calls += 1;
            calls == stop
        });
        assert_eq!(
            result,
            Err(evaluation_failure(
                super::super::super::RootIsolationError::Cancelled
            ))
        );
        assert_eq!(calls, stop);
    }
}
#[test]
fn empty_and_all_point_inputs_keep_542_results_and_exact_poll_counts() {
    for f in [
        fixture(&[0., 0., 1., 0., -2., 0., 1.], &[0., 1.], [-1., 1.], 0.001),
        fixture(&[1., 0., 1.], &[2.], [-1., 1.], 0.001),
    ] {
        let mut calls = 0;
        let current = run(&f, &mut || {
            calls += 1;
            false
        })
        .unwrap();
        let old = eager(&f);
        assert_eq!((current, calls), old);
        assert_all_cancellations(&f);
    }
}
