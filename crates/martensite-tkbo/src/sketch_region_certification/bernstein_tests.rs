// SPDX-License-Identifier: GPL-2.0-only
//! Issue #539: independently monotone binary64 source, real fallback and stale evidence.
use super::paired_tests::{bezier_shape, curved_pair};
use super::source_binding_tests::source_policy;
use super::tests::{capture, policy};
use super::*;
use martensite_tkmath::predicates::roots::bezier_pair::*;

pub(super) fn radial_source() -> martensite_tkbrep::topods::Shape {
    let x = [
        0,
        4599676419421066576,
        4604180019048437080,
        4607182418800017408,
    ]
    .map(f64::from_bits);
    assert!(x.windows(2).all(|pair| pair[0] < pair[1]));
    bezier_shape(&x.map(|value| [value, 0.]), None)
}
fn excluded() -> BoundaryCensus {
    enumerate_exclusion_census(
        capture(&[radial_source()]),
        policy(),
        source_policy(),
        || false,
    )
    .unwrap_or_else(|_| panic!("retained exact census"))
}
#[test]
fn exact_exclusion_completes_the_owned_monotone_cubic_without_legacy_evidence() {
    let census = excluded();
    let paired = census.paired.as_ref().unwrap();
    assert_eq!(paired.exclusion_version, Some(1));
    assert!(paired.reports.is_empty());
    let record = &paired.exclusions[0];
    let execution = record.outcome.as_ref().unwrap();
    assert_eq!(
        execution.exclusion.completion(),
        super::conditioned_stage::ConditionedCompletion::Complete
    );
    assert!(execution.fallback.is_none());
    assert_eq!(census.bisections, record.completed_work());
    assert_complete_empty_domain(&census);
}
pub(super) fn assert_complete_empty_domain(census: &BoundaryCensus) {
    let certificates = DomainCertificates::new(census).unwrap();
    let proof = certificates
        .unified_certificate(&census.domains[0])
        .unwrap();
    assert_eq!(proof.contact_count(), 0);
    assert!(!proof.common_carrier());
    assert!(certificates.certificate(&census.domains[0]).is_none());
    assert!(
        !census
            .current_obligations()
            .contains(&CertificationObligation::ExhaustivePairAndSelfDomains)
    );
}
#[test]
fn released_carrier_census_keeps_the_original_non_linear_fiber_refusal() {
    let census = enumerate_carrier_census(
        capture(&[radial_source()]),
        policy(),
        source_policy(),
        || false,
    )
    .unwrap_or_else(|_| panic!("retained released census"));
    let paired = census.paired.as_ref().unwrap();
    assert_eq!(paired.exclusion_version, None);
    assert!(paired.exclusions.is_empty());
    let report = paired.reports[0].outcome.as_ref().unwrap();
    let Some(BezierPairFailure::Operation(cause)) = report.failure() else {
        panic!("original operation refusal")
    };
    assert!(matches!(
        cause.original_cause(),
        BezierPairOperationFailure::NonLinearFiber
    ));
}
#[test]
fn inconclusive_exact_hull_retains_the_genuine_crossing_solver() {
    let census =
        enumerate_exclusion_census(capture(&curved_pair()), policy(), source_policy(), || false)
            .unwrap_or_else(|_| panic!("retained crossing census"));
    let paired = census.paired.as_ref().unwrap();
    let record = paired
        .exclusions
        .iter()
        .find(|record| record.mode == super::paired_types::PairedQueryMode::Distinct)
        .unwrap();
    assert_crossing_fallback(record);
    assert_complete_crossing_domain(&census);
}
pub(super) fn assert_complete_crossing_domain(census: &BoundaryCensus) {
    let paired = census.paired.as_ref().unwrap();
    let consumed: u64 = paired
        .exclusions
        .iter()
        .map(|record| record.completed_work())
        .sum();
    assert_eq!(census.bisections, consumed);
    let certificates = DomainCertificates::new(census).unwrap();
    assert!(
        certificates
            .unified_certificate(&census.domains[1])
            .is_some()
    );
}
pub(super) fn assert_crossing_fallback(record: &super::bernstein_execution::BernsteinStoredReport) {
    let execution = record.outcome.as_ref().unwrap();
    assert_eq!(
        execution.exclusion.completion(),
        super::conditioned_stage::ConditionedCompletion::Inconclusive
    );
    let fallback = execution.fallback.as_ref().unwrap();
    assert_eq!(fallback.complete().unwrap().contact_count(), 1);
    assert_eq!(fallback.source(), execution.exclusion.source());
    assert_eq!(
        fallback.policy().roots.max_bisections,
        record.effective.roots.max_bisections - execution.exclusion.work().completed
    );
}

#[test]
fn exact_domain_refuses_mutated_source_domain_and_parameter_binding() {
    for mutation in 0..4 {
        let mut census = excluded();
        mutate_domain_source(&mut census, mutation);
        let proof = DomainCertificates::new(&census)
            .and_then(|view| view.unified_certificate(&census.domains[0]));
        assert!(proof.is_none());
    }
}
fn mutate_domain_source(census: &mut BoundaryCensus, mutation: usize) {
    match mutation {
        0 => census.inputs.segments[0].controls[1][0] += 0.01,
        1 => census.inputs.segments[0].interval[1] = 0.5,
        2 => census.paired.as_mut().unwrap().exclusion_version = Some(2),
        _ => {
            let DomainEvidence::BernsteinReport { source_slots, .. } =
                &mut census.domains[0].evidence
            else {
                panic!("exact record")
            };
            *source_slots = [1, 0];
        }
    }
}

#[test]
fn exact_domain_refuses_policy_and_work_ledger_mutation() {
    for mutation in 0..4 {
        let mut census = excluded();
        let paired = census.paired.as_mut().unwrap();
        match mutation {
            0 => paired.requested.roots.max_bisections -= 1,
            1 => paired.requested.signs.version = 99,
            2 => paired.reused_reports += 1,
            _ => census.bisections += 1,
        }
        assert!(DomainCertificates::new(&census).is_none());
    }
}
#[test]
fn cancelled_exact_execution_keeps_actual_work_and_no_fallback() {
    let mut census = excluded();
    let paired = census.paired.as_mut().unwrap();
    paired.exclusions.clear();
    census.bisections = 0;
    let mut reports = super::paired_execution::test_report_lookup(&census);
    let evidence = super::bernstein_execution::evaluate(
        &mut census,
        [0, 0],
        [0, 0],
        super::paired_types::PairedQueryMode::SelfOffDiagonal,
        &mut || true,
        &mut reports,
        &super::paired_execution::PairSourcePool::default(),
    );
    let DomainEvidence::BernsteinReport { report, .. } = evidence else {
        panic!("actual exact execution")
    };
    let record = &census.paired.as_ref().unwrap().exclusions[report];
    assert!(record.cancelled());
    assert!(record.outcome.as_ref().unwrap().fallback.is_none());
    assert_eq!(census.bisections, record.completed_work());
}
#[test]
fn zero_budget_is_never_increased_for_exact_proof_or_solver_fallback() {
    let mut requested = policy();
    requested.roots.max_bisections = 0;
    let census =
        enumerate_exclusion_census(capture(&curved_pair()), requested, source_policy(), || {
            false
        })
        .unwrap_or_else(|_| panic!("retained bounded census"));
    assert_eq!(census.bisections, 0);
    for record in &census.paired.as_ref().unwrap().exclusions {
        assert_eq!(record.effective.roots.max_bisections, 0);
        if let Some(fallback) = &record.outcome.as_ref().unwrap().fallback {
            assert_eq!(fallback.policy().roots.max_bisections, 0);
        }
    }
}
