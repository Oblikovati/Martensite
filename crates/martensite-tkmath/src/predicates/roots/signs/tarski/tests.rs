// SPDX-License-Identifier: GPL-2.0-only
//! #542: exact-point signs avoid unused query chains; mixed roots retain eager execution.
use super::*;
use crate::predicates::roots::{
    RootIsolationError, RootIsolationOptions, isolate_polynomial_roots,
};
struct Fixture {
    source: IntegerPolynomial,
    query: IntegerPolynomial,
    isolation: RootIsolation,
}
fn fixture(source: &[f64], query: &[f64], bounds: [f64; 2]) -> Fixture {
    let options = RootIsolationOptions::new(0.001);
    let isolation = isolate_polynomial_roots(source, bounds, options, || false).unwrap();
    let source = IntegerPolynomial::from_binary64(source, 32).unwrap();
    let query =
        IntegerPolynomial::accept_zero(IntegerPolynomial::from_binary64(query, 32)).unwrap();
    Fixture {
        source,
        query,
        isolation,
    }
}
fn repeated_points(query: &[f64]) -> Fixture {
    fixture(&[0., 0., 1., 0., -2., 0., 1.], query, [-1., 1.])
}
fn current(
    fixture: &Fixture,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<Ordering>, RootSignError> {
    evaluate(
        fixture.source.clone(),
        fixture.query.clone(),
        &fixture.isolation,
        cancelled,
    )
}
fn evaluate_counted(fixture: &Fixture) -> (Vec<Ordering>, usize) {
    let mut polls = 0;
    let result = current(fixture, &mut || {
        polls += 1;
        false
    })
    .unwrap();
    (result, polls)
}
fn assert_singleton_fixture(fixture: &Fixture) {
    assert!(
        fixture
            .isolation
            .roots()
            .iter()
            .all(|root| root.bounds().0 == root.bounds().1)
    );
}
#[test]
fn singleton_signs_keep_exact_order_multiplicity_and_isolation() {
    let fixture = repeated_points(&[0., 1.]);
    let before = fixture.isolation.clone();
    assert_singleton_fixture(&fixture);
    assert_eq!(
        evaluate_counted(&fixture).0,
        [Ordering::Less, Ordering::Equal, Ordering::Greater]
    );
    assert_eq!(fixture.isolation, before);
    let multiplicities: Vec<_> = fixture
        .isolation
        .roots()
        .iter()
        .map(RootBracket::multiplicity)
        .collect();
    assert_eq!(multiplicities, [2, 2, 2]);
}
#[test]
fn empty_isolation_still_polls_before_returning_complete_empty() {
    let fixture = fixture(&[1., 0., 1.], &[5., -7., 3.], [-1., 1.]);
    assert!(evaluate_counted(&fixture).0.is_empty());
    assert_evaluation_cancelled(&fixture, 1);
}
#[test]
fn singleton_zero_constant_and_subnormal_queries_use_exact_signs() {
    for (query, expected) in [
        (vec![], vec![Ordering::Equal; 3]),
        (vec![-3.], vec![Ordering::Less; 3]),
        (
            vec![0., f64::from_bits(1)],
            vec![Ordering::Less, Ordering::Equal, Ordering::Greater],
        ),
    ] {
        assert_eq!(evaluate_counted(&repeated_points(&query)).0, expected);
    }
}
#[test]
fn every_singleton_checkpoint_cancels_without_partial_signs() {
    for fixture in [repeated_points(&[0., 1.]), repeated_points(&[])] {
        let (_, checkpoints) = evaluate_counted(&fixture);
        for stop in 1..=checkpoints {
            assert_evaluation_cancelled(&fixture, stop);
        }
    }
}
fn assert_evaluation_cancelled(fixture: &Fixture, stop: usize) {
    let mut calls = 0;
    let result = current(fixture, &mut || {
        calls += 1;
        calls == stop
    });
    assert_eq!(
        result,
        Err(RootSignStage::Evaluation.failure(RootIsolationError::Cancelled))
    );
    assert_eq!(calls, stop);
}
fn eager(
    fixture: &Fixture,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<Ordering>, RootSignError> {
    let chain = query_chain(fixture.source.clone(), &fixture.query, cancelled)
        .map_err(|cause| RootSignStage::Evaluation.failure(cause))?;
    fixture
        .isolation
        .roots()
        .iter()
        .map(|root| classify(&chain, &fixture.query, root, cancelled))
        .collect()
}
fn call(
    fixture: &Fixture,
    old: bool,
    stop: usize,
) -> (Result<Vec<Ordering>, RootSignError>, usize) {
    let mut calls = 0;
    let mut cancel = || {
        calls += 1;
        calls == stop
    };
    let result = if old {
        eager(fixture, &mut cancel)
    } else {
        current(fixture, &mut cancel)
    };
    (result, calls)
}
fn assert_eager_unchanged(fixture: Fixture, expected: &[Ordering]) {
    let old = call(&fixture, true, usize::MAX);
    assert_eq!(old.0.as_ref().unwrap(), expected);
    assert_eq!(call(&fixture, false, usize::MAX), old);
    for stop in 1..=old.1 {
        assert_eq!(call(&fixture, false, stop), call(&fixture, true, stop));
    }
}
#[test]
fn mixed_point_then_irrational_root_keeps_entire_eager_execution() {
    let fixture = fixture(&[0., -2., 0., 1.], &[0., 1.], [0., 2.]);
    let roots = fixture.isolation.roots();
    assert_eq!(roots[0].bounds().0, roots[0].bounds().1);
    assert_ne!(roots[1].bounds().0, roots[1].bounds().1);
    assert_eager_unchanged(fixture, &[Ordering::Equal, Ordering::Greater]);
}
#[test]
fn irrational_only_roots_keep_entire_eager_execution() {
    let fixture = fixture(&[-2., 0., 1.], &[0., 1.], [-2., 2.]);
    assert!(
        fixture
            .isolation
            .roots()
            .iter()
            .all(|root| root.bounds().0 != root.bounds().1)
    );
    assert_eager_unchanged(fixture, &[Ordering::Less, Ordering::Greater]);
}
#[test]
fn point_and_empty_domains_still_refuse_unadmitted_query_degrees() {
    use crate::predicates::roots::signs::signs_at_polynomial_roots;
    let mut policy = RootIsolationOptions::new(0.001);
    policy.max_degree = 1;
    for source in [&[0., 1.][..], &[1.][..]] {
        let result = signs_at_polynomial_roots(source, &[0., 0., 1.], [-1., 1.], policy, || false);
        assert_eq!(
            result,
            Err(RootSignStage::Query.failure(RootIsolationError::Degree {
                actual: 2,
                limit: 1
            }))
        );
    }
}
#[test]
fn singleton_domain_still_refuses_invalid_policy_before_cancellation() {
    use crate::predicates::roots::signs::signs_at_polynomial_roots;
    let mut policy = RootIsolationOptions::new(0.001);
    policy.version = 2;
    let result = signs_at_polynomial_roots(&[0., 1.], &[1.], [-1., 1.], policy, || true);
    assert_eq!(
        result,
        Err(RootSignStage::Policy.failure(RootIsolationError::OptionsVersion(2)))
    );
}
#[test]
fn invalid_tarski_sums_are_declines_in_both_directions() {
    for (lower, upper) in [(2, 0), (0, 2), (usize::MAX, 0)] {
        assert_eq!(
            certified_difference(lower, upper),
            Err(RootSignError::Variation { lower, upper })
        );
    }
}
