// SPDX-License-Identifier: GPL-2.0-only
//! Geometric leaves carry exact endpoint evidence through the existing Census boundary.
use super::paired_tests::{bezier_shape, curved_pair};
use super::source_binding_tests::source_policy;
use super::tests::{capture, policy};
use super::*;

pub(super) fn endpoint_census() -> BoundaryCensus {
    endpoint_census_with_cancel(|| false).unwrap_or_else(|_| panic!("geometric Census"))
}
fn endpoint_census_with_cancel(
    cancelled: impl FnMut() -> bool,
) -> Result<BoundaryCensus, CensusCause> {
    let left = bezier_shape(&[[0., 0.], [0.25, 0.], [0.75, 0.], [1., 0.]], None);
    let right = bezier_shape(&[[1., 0.], [1.25, 0.25], [1.75, 0.75], [2., 1.]], None);
    enumerate_geometric_census_with_access(
        capture(&[left, right]),
        policy(),
        source_policy(),
        cancelled,
        SourceAccess::Live,
    )
}
#[test]
fn geometric_census_issues_endpoint_evidence_without_solver_roots() {
    let census = endpoint_census();
    let view = DomainCertificates::new(&census).unwrap();
    let domain = view.unified_certificate(&census.domains[1]).unwrap();
    let contact = domain.contact(0).unwrap();
    assert_eq!(contact.exact_endpoints(), Some([1, 0]));
    assert_eq!(contact.point(), [1., 0.]);
    assert_eq!(contact.key(), ContactProofKey::Endpoint);
    assert_eq!(contact.projected_multiplicity(), None);
    assert!(domain.solver().is_none());
    assert_eq!(census.bisections, 0);
    assert!(
        census
            .paired
            .as_ref()
            .unwrap()
            .exclusions
            .iter()
            .all(|record| { record.outcome.as_ref().unwrap().fallback.is_none() })
    );
    assert!(
        !census
            .current_obligations()
            .contains(&CertificationObligation::ExhaustivePairAndSelfDomains)
    );
}
#[test]
fn geometric_census_retains_genuine_general_fallback_and_charges_work_once() {
    let census = enumerate_geometric_census_with_access(
        capture(&curved_pair()),
        policy(),
        source_policy(),
        || false,
        SourceAccess::Live,
    )
    .unwrap_or_else(|_| panic!("fallback Census"));
    super::bernstein_tests::assert_complete_crossing_domain(&census);
    let view = DomainCertificates::new(&census).unwrap();
    assert!(
        view.unified_certificate(&census.domains[1])
            .unwrap()
            .solver()
            .is_some()
    );
}
#[test]
fn geometric_census_rejects_changed_source_policy_and_parameter_axis() {
    for mutation in 0..4 {
        let mut census = endpoint_census();
        mutate(&mut census, mutation);
        let view = DomainCertificates::new(&census);
        assert!(
            view.and_then(|view| view.unified_certificate(&census.domains[1]))
                .is_none()
        );
    }
}
fn mutate(census: &mut BoundaryCensus, mutation: usize) {
    match mutation {
        0 => census.inputs.segments[0].controls[1][0] += 0.125,
        1 => census.paired.as_mut().unwrap().exclusion_version = Some(1),
        2 => census.inputs.segments[0].interval[1] = 0.5,
        _ => {
            let DomainEvidence::BernsteinReport { source_slots, .. } =
                &mut census.domains[1].evidence
            else {
                panic!("conditioned evidence")
            };
            *source_slots = [1, 0];
        }
    }
}

#[test]
fn geometric_census_cancellation_never_publishes_complete_pair_coverage() {
    let mut total = 0;
    endpoint_census_with_cancel(|| {
        total += 1;
        false
    })
    .unwrap_or_else(|_| panic!("uncancelled geometric Census"));
    assert!(total > 2);
    for cut in [total - 2, total - 1, total] {
        assert_cancelled_at(cut);
    }
}
fn assert_cancelled_at(cut: usize) {
    let mut polls = 0;
    let outcome = endpoint_census_with_cancel(|| {
        polls += 1;
        polls == cut
    });
    let completed = match outcome {
        Err(CensusCause::Cancelled { completed, .. }) => completed,
        _ => panic!("cancelled at poll {cut}"),
    };
    assert_eq!(polls, cut);
    assert!(!super::paired_binding::paired_domains_certified(&completed));
    assert_eq!(completed.bisections, 0);
}
