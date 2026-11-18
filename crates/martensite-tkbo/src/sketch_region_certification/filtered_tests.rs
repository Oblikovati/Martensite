// SPDX-License-Identifier: GPL-2.0-only
//! #544: selector provenance and legacy replay use the actual native-source census.
use super::paired_tests::curved_pair;
use super::source_binding_tests::source_policy;
use super::tests::{capture, policy};
use super::*;

#[test]
fn legacy_bernstein_keeps_its_native_poll_work_signature() {
    let mut polls = 0_u64;
    let census =
        enumerate_exclusion_census(capture(&curved_pair()), policy(), source_policy(), || {
            polls += 1;
            false
        })
        .unwrap_or_else(|_| panic!("legacy Bernstein census"));
    let paired = census.paired.as_ref().unwrap();
    let signature = [
        polls,
        census.bisections,
        paired.reused_reports,
        paired.exclusions.len() as u64,
        census.domains.len() as u64,
    ];
    // #544: sign-1 replay is pinned to the original eager source 54e2df1a.
    assert_eq!(signature, [1754, 1, 0, 3, 3]);
    assert_eq!(paired.requested.signs.version, 1);
}

use super::bernstein_tests::{
    assert_complete_crossing_domain, assert_complete_empty_domain, assert_crossing_fallback,
    radial_source,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::predicates::roots::bezier_pair::*;
fn filtered(sources: &[Shape]) -> BoundaryCensus {
    enumerate_filtered_exclusion_census(capture(sources), policy(), source_policy(), || false)
        .unwrap_or_else(|_| panic!("filtered native census"))
}
#[test]
fn filtered_native_exclusion_retains_actual_policy_and_domain_cardinality() {
    let census = filtered(&[radial_source()]);
    let paired = census.paired.as_ref().unwrap();
    assert_eq!(paired.selected_signs, paired.requested.signs);
    assert_eq!(paired.requested.signs.version, 2);
    assert_eq!(paired.exclusion_version, Some(1));
    assert!(paired.reports.is_empty());
    let record = &paired.exclusions[0];
    let execution = record.outcome.as_ref().unwrap();
    assert_eq!(execution.exclusion.policy().pair_policy, record.effective);
    assert!(execution.fallback.is_none());
    assert_eq!(census.domains.len(), 1);
    assert_eq!(census.bisections, record.completed_work());
    assert_complete_empty_domain(&census);
}
#[test]
fn filtered_native_crossing_keeps_the_real_fallback_and_remaining_policy() {
    let census = filtered(&curved_pair());
    let paired = census.paired.as_ref().unwrap();
    let record = paired
        .exclusions
        .iter()
        .find(|record| record.mode == super::paired_types::PairedQueryMode::Distinct)
        .unwrap();
    assert_crossing_fallback(record);
    let execution = record.outcome.as_ref().unwrap();
    let fallback = execution.fallback.as_ref().unwrap();
    assert_eq!(fallback.policy().signs.version, 2);
    let graph = fallback.complete().unwrap().graph().unwrap();
    assert_eq!(graph.effective_signs().requested, paired.selected_signs);
    assert_eq!(
        execution.exclusion.policy().pair_policy.signs,
        paired.selected_signs
    );
    assert_complete_crossing_domain(&census);
}
#[test]
fn filtered_receipts_reject_source_policy_and_mode_mutation() {
    for mutation in 0..9 {
        let mut census = filtered(&[radial_source()]);
        mutate(&mut census, mutation);
        let certificate = DomainCertificates::new(&census)
            .and_then(|view| view.unified_certificate(&census.domains[0]));
        assert!(certificate.is_none(), "mutation={mutation}");
    }
}
fn mutate(census: &mut BoundaryCensus, mutation: usize) {
    let paired = census.paired.as_mut().unwrap();
    match mutation {
        0 => census.inputs.segments[0].controls[1][0] += 0.01,
        1 => census.inputs.segments[0].interval[1] = 0.5,
        2 => paired.selected_signs.version = 1,
        3 => paired.requested.signs.version = 1,
        4 => paired.exclusions[0].effective.signs.version = 1,
        5 => paired.selected_signs.degree_limit = BezierPairSignAdmission::Explicit(0),
        6 => paired.requested.version = 99,
        7 => paired.exclusion_version = Some(2),
        _ => paired.exclusions[0].sources = [1, 1],
    }
}
#[test]
fn unknown_filtered_sign_mode_and_old_mode_relabeling_are_refused() {
    let mut census = filtered(&[radial_source()]);
    let paired = census.paired.as_mut().unwrap();
    paired.selected_signs.version = 3;
    paired.requested.signs.version = 3;
    assert!(DomainCertificates::new(&census).is_none());
    let mut legacy = enumerate_exclusion_census(
        capture(&[radial_source()]),
        policy(),
        source_policy(),
        || false,
    )
    .unwrap_or_else(|_| panic!("legacy census"));
    let paired = legacy.paired.as_mut().unwrap();
    paired.selected_signs.version = 2;
    paired.requested.signs.version = 2;
    assert!(DomainCertificates::new(&legacy).is_none());
}
#[test]
fn filtered_zero_work_keeps_version_two_in_actual_fallback_requests() {
    let mut requested = policy();
    requested.roots.max_bisections = 0;
    let census = enumerate_filtered_exclusion_census(
        capture(&curved_pair()),
        requested,
        source_policy(),
        || false,
    )
    .unwrap_or_else(|_| panic!("bounded filtered census"));
    assert_eq!(census.bisections, 0);
    for record in &census.paired.as_ref().unwrap().exclusions {
        assert_eq!(record.effective.signs.version, 2);
        assert_eq!(record.effective.roots.max_bisections, 0);
        if let Some(fallback) = &record.outcome.as_ref().unwrap().fallback {
            assert_eq!(fallback.policy().signs.version, 2);
            assert_eq!(fallback.policy().roots.max_bisections, 0);
        }
    }
}
