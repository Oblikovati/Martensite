// SPDX-License-Identifier: GPL-2.0-only
//! Run the literal second transform at every callback boundary as the reference.
use super::super::{
    super::RootIsolationError,
    operation::{AccountedPairOutcome, compute_with_signs, validate_world_request},
    sign_policy::{PairSignOptions, PairSignPolicyFailure},
    source::PairSourceFailure,
    source_certificate::certify as certify_literal,
};
use super::*;
use std::sync::Arc;

#[derive(Clone, Copy)]
struct Policy {
    roots: RootIsolationOptions,
    signs: PairSignOptions,
    world_error: f64,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            roots: RootIsolationOptions::new(0.01),
            signs: PairSignOptions::derived(),
            world_error: 1e-6,
        }
    }
}

fn source(controls: &[[f64; 2]], weights: Option<&[f64]>) -> OwnedBezierSource {
    OwnedBezierSource::capture(controls, weights, [0., 1.]).unwrap()
}

fn collinear_pair() -> [OwnedBezierSource; 2] {
    [
        source(&[[0., 0.], [1., 0.]], None),
        source(&[[1., 0.], [2., 0.]], None),
    ]
}

fn rational_pair() -> [OwnedBezierSource; 2] {
    [
        source(&[[1., 0.], [1., 1.], [0., 1.]], Some(&[1., 1., 2.])),
        source(&[[0., 1.], [-1., 1.], [-1., 0.]], Some(&[1., 1., 2.])),
    ]
}

fn crossing_pair() -> [OwnedBezierSource; 2] {
    [
        source(&[[0., 0.], [0.5, 0.], [1., 1.]], None),
        source(&[[0., 0.5], [0.5, 0.5], [1., -0.5]], None),
    ]
}

#[test]
fn rational_carrier_and_generic_graph_preserve_every_callback_boundary() {
    let common = audit(rational_pair(), Policy::default());
    let generic = audit(crossing_pair(), Policy::default());
    assert_one_contact(common, [0., 1.], true);
    assert_one_contact(generic, [0.5, 0.25], false);
}

#[test]
fn nonfinite_sources_and_invalid_world_request_keep_admission_order() {
    let mut sources = collinear_pair();
    sources[1] = source(&[[1., 0.], [f64::INFINITY, 0.]], None);
    let right = audit(sources.clone(), Policy::default());
    assert!(matches!(
        right.completion,
        Err(PairOperationFailure::Source(PairSourceFailure::Coordinate(
            _
        )))
    ));
    sources[0] = source(&[[0., 0.], [1., 0.]], Some(&[1., f64::NAN]));
    let left = audit(sources.clone(), Policy::default());
    assert!(matches!(
        left.completion,
        Err(PairOperationFailure::Source(PairSourceFailure::Weight {
            index: 1,
            ..
        }))
    ));
    let policy = Policy {
        world_error: 0.,
        ..Policy::default()
    };
    let world = audit(sources, policy);
    assert!(matches!(
        world.completion,
        Err(PairOperationFailure::WorldRequest(0.))
    ));
}

#[test]
fn root_and_sign_policies_keep_common_and_generic_diagnostics() {
    let mut policy = Policy::default();
    policy.roots.version = 7;
    let common = audit(collinear_pair(), policy);
    let generic = audit(crossing_pair(), policy);
    assert!(matches!(
        common.completion,
        Err(PairOperationFailure::Root(
            RootIsolationError::OptionsVersion(7)
        ))
    ));
    assert_domain_error(generic, 0, |cause| {
        matches!(
            cause,
            PairOperationFailure::Root(RootIsolationError::OptionsVersion(7))
        )
    });

    policy.roots.version = 1;
    policy.signs.version = 7;
    assert_one_contact(audit(collinear_pair(), policy), [1., 0.], true);
    let generic = audit(crossing_pair(), policy);
    assert!(matches!(
        generic.completion,
        Err(PairOperationFailure::SignPolicy(
            PairSignPolicyFailure::Version(7)
        ))
    ));
}

#[test]
fn denominator_failure_and_partial_work_keep_distinct_source_contexts() {
    let mut common_sources = collinear_pair();
    common_sources[0] = source(&[[0., 0.], [1., 0.]], Some(&[1., -2.]));
    let common = audit(common_sources, Policy::default());
    assert!(matches!(
        common.completion,
        Err(PairOperationFailure::DenominatorRoots(_))
    ));
    assert!(common.work.completed > 0);

    let mut generic_sources = crossing_pair();
    generic_sources[1] = source(&[[0., 0.5], [0.5, 0.5], [1., -0.5]], Some(&[1., 1., -2.]));
    let mut policy = Policy::default();
    policy.roots.max_bisections = 1;
    let generic = audit(generic_sources, policy);
    assert_eq!(generic.work.completed, 1);
    assert_domain_error(generic, 1, |cause| {
        matches!(
            cause,
            PairOperationFailure::Root(RootIsolationError::SubdivisionLimit { limit: 1 })
        )
    });
}

fn assert_one_contact(outcome: AccountedPairOutcome, point: [f64; 2], common: bool) {
    let proof = outcome.completion.unwrap();
    assert_eq!(proof.contacts.len(), 1);
    assert_eq!(proof.contacts[0].world.point, point);
    assert_eq!(
        matches!(proof.evidence, PairProofEvidence::CommonCarrier(_)),
        common
    );
}

fn assert_domain_error(
    outcome: AccountedPairOutcome,
    index: usize,
    expected: impl FnOnce(&PairOperationFailure) -> bool,
) {
    let Err(PairOperationFailure::SourceDomain {
        source_index,
        cause,
    }) = outcome.completion
    else {
        panic!("expected source-domain failure at {index}");
    };
    assert_eq!(source_index, index);
    assert!(expected(&cause), "unexpected source-domain cause {cause:?}");
}

fn audit(sources: [OwnedBezierSource; 2], policy: Policy) -> AccountedPairOutcome {
    let (count, outcome) = compare_at(&sources, policy, None);
    for stop in 1..=count {
        let (called, cancelled) = compare_at(&sources, policy, Some(stop));
        assert_eq!(called, stop);
        let Err(cause) = cancelled.completion else {
            panic!("expected cancellation at callback {stop} of {count}");
        };
        assert_eq!(
            super::super::completion::operation_completion(&cause),
            super::super::BezierPairCompletion::Cancelled
        );
    }
    outcome
}

fn compare_at(
    sources: &[OwnedBezierSource; 2],
    policy: Policy,
    stop: Option<usize>,
) -> (usize, AccountedPairOutcome) {
    let mut literal_calls = 0;
    let literal = literal_carrier(sources, policy, &mut || {
        literal_calls += 1;
        Some(literal_calls) == stop
    });
    let mut actual_calls = 0;
    let actual = compute_with_signs(
        sources.clone(),
        (false, true),
        policy.roots,
        policy.signs,
        policy.world_error,
        &mut || {
            actual_calls += 1;
            Some(actual_calls) == stop
        },
    );
    assert_eq!(actual_calls, literal_calls, "callback stop {stop:?}");
    assert_eq!(actual.work, literal.work, "callback stop {stop:?}");
    assert_same_completion(&actual.completion, &literal.completion, stop);
    (actual_calls, actual)
}

fn literal_carrier(
    sources: &[OwnedBezierSource; 2],
    policy: Policy,
    cancelled: &mut impl FnMut() -> bool,
) -> AccountedPairOutcome {
    let budget = PairWorkBudget::new(policy.roots.max_bisections);
    match literal_common(sources, policy, &budget, cancelled) {
        Ok(None) => compute_with_signs(
            sources.clone(),
            (false, false),
            policy.roots,
            policy.signs,
            policy.world_error,
            cancelled,
        ),
        completion => AccountedPairOutcome {
            completion: completion.map(Option::unwrap),
            work: budget.receipt(),
        },
    }
}

fn literal_common(
    sources: &[OwnedBezierSource; 2],
    policy: Policy,
    budget: &PairWorkBudget,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Option<PairOperationProof>, PairOperationFailure> {
    validate_world_request(policy.world_error)?;
    let left = sources[0]
        .homogeneous(cancelled)
        .map_err(PairOperationFailure::Source)?;
    let right = sources[1]
        .homogeneous(cancelled)
        .map_err(PairOperationFailure::Source)?;
    let Some(carrier) = shared_carrier(&left, &right) else {
        return Ok(None);
    };
    let certificates = [
        certify_literal(&sources[0], policy.roots, budget, cancelled)?,
        certify_literal(&sources[1], policy.roots, budget, cancelled)?,
    ];
    assert_eq!(certificates[1].homogeneous, right);
    common_proof(
        sources,
        carrier,
        certificates,
        policy.world_error,
        budget,
        cancelled,
    )
    .map(Some)
}

fn assert_same_completion(
    actual: &Result<PairOperationProof, PairOperationFailure>,
    literal: &Result<PairOperationProof, PairOperationFailure>,
    stop: Option<usize>,
) {
    match (actual, literal) {
        (Ok(actual), Ok(literal)) => assert_same_proof(actual, literal),
        (Err(actual), Err(literal)) => assert_eq!(
            format!("{actual:?}"),
            format!("{literal:?}"),
            "callback stop {stop:?}"
        ),
        _ => panic!("different completion at callback stop {stop:?}"),
    }
}

fn assert_same_proof(actual: &PairOperationProof, literal: &PairOperationProof) {
    assert_eq!(actual.work, literal.work);
    for (a, b) in actual.sources.iter().zip(&literal.sources) {
        assert_eq!(a, b);
        assert!(Arc::ptr_eq(&a.controls, &b.controls));
        if let (Some(a), Some(b)) = (&a.weights, &b.weights) {
            assert!(Arc::ptr_eq(a, b));
        }
    }
    for (a, b) in actual
        .source_certificates
        .iter()
        .zip(&literal.source_certificates)
    {
        assert_eq!(a.homogeneous, b.homogeneous);
        assert_eq!(a.denominator, b.denominator);
    }
    assert_same_evidence(&actual.evidence, &literal.evidence);
    assert_eq!(actual.contacts.len(), literal.contacts.len());
    for (a, b) in actual.contacts.iter().zip(&literal.contacts) {
        assert_same_contact(a, b);
    }
}

fn assert_same_contact(a: &PairContact, b: &PairContact) {
    assert_eq!(a.candidate_index, b.candidate_index);
    assert_eq!(a.root, b.root);
    assert_eq!(a.u_bounds.map(f64::to_bits), b.u_bounds.map(f64::to_bits));
    assert_eq!(a.exact_u.lower, b.exact_u.lower);
    assert_eq!(a.exact_u.upper, b.exact_u.upper);
    let bits = |p: [f64; 2]| p.map(f64::to_bits);
    assert_eq!(a.world.bounds.map(bits), b.world.bounds.map(bits));
    assert_eq!(bits(a.world.point), bits(b.world.point));
    assert_eq!(
        a.world.achieved_error.to_bits(),
        b.world.achieved_error.to_bits()
    );
    assert_eq!(a.world.error_squared, b.world.error_squared);
}

fn assert_same_evidence(actual: &PairProofEvidence, literal: &PairProofEvidence) {
    match (actual, literal) {
        (PairProofEvidence::CommonCarrier(a), PairProofEvidence::CommonCarrier(b)) => {
            assert_eq!(a.kind, b.kind);
            assert_eq!(a.preimages, b.preimages);
        }
        (PairProofEvidence::Graph(a), PairProofEvidence::Graph(b)) => {
            assert_eq!(a.projection, b.projection);
            assert_eq!(a.signs, b.signs);
            assert_eq!(a.exceptional_fibers, b.exceptional_fibers);
            let (a, b) = (&a.candidate_certificate, &b.candidate_certificate);
            assert_eq!(a.polynomial, b.polynomial);
            assert_eq!(a.isolation, b.isolation);
            assert_eq!(a.membership.accepted, b.membership.accepted);
            assert_eq!(a.membership.queries.len(), b.membership.queries.len());
            for (a, b) in a.membership.queries.iter().zip(&b.membership.queries) {
                assert_eq!(a.query.rule, b.query.rule);
                assert_eq!(a.query.polynomial, b.query.polynomial);
                assert_eq!(a.evaluation, b.evaluation);
            }
        }
        (PairProofEvidence::ConstantExcluded(a), PairProofEvidence::ConstantExcluded(b)) => {
            assert_eq!(a, b)
        }
        _ => panic!("different evidence route"),
    }
}
