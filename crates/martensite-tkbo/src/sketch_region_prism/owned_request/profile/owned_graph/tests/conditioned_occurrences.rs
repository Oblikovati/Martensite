// SPDX-License-Identifier: GPL-2.0-only
//! #572 retains only occurrence metadata for actual V3–V6 unified certificates.
use super::*;
use crate::sketch_region_certification::{CertifiedPairDomain, DomainCertificates};
use std::cell::Cell;
mod refusals;

fn prepared(version: u32) -> super::super::super::PreparedProfile {
    let mut request = request();
    request.version = version;
    request.integral_program = (version >= 5).then(program::options);
    super::super::super::pipeline::prepare_owned_profile(request, &mut Continue)
        .unwrap_or_else(|_| panic!("actual V{version} occurrence fixture preparation"))
}
fn signature(view: &DomainCertificates<'_>, census: &BoundaryCensus) -> Vec<String> {
    census
        .domains
        .iter()
        .map(|domain| {
            domain_signature(
                view.unified_certificate(domain).unwrap(),
                domain.occurrences,
            )
        })
        .collect()
}
fn domain_signature(proof: CertifiedPairDomain<'_>, occurrences: [usize; 2]) -> String {
    let kind = match &proof {
        CertifiedPairDomain::Solver(_) => "solver",
        CertifiedPairDomain::Geometric(_) => "geometric",
        CertifiedPairDomain::Excluded { .. } => "excluded",
    };
    let contacts = (0..proof.contact_count())
        .map(|index| {
            let contact = proof.contact(index).unwrap();
            (
                contact.point().map(f64::to_bits),
                contact.exact_endpoints(),
                contact.key(),
            )
        })
        .collect::<Vec<_>>();
    format!("{kind}:{occurrences:?}:{contacts:?}")
}

#[test]
fn conditioned_owned_views_count_initial_live_eligibility_then_reuse_only_occurrence_checks() {
    for version in 3..=6 {
        check_view(version);
    }
}
fn check_view(version: u32) {
    let prepared = prepared(version);
    let census = &prepared.bridge.census;
    let captures = Cell::new(0);
    let owner = OwnedGraphAccess::fresh(&prepared.request, &prepared.edges, &captures);
    let owned = SourceAccess::Owned(&owner).certificates(census).unwrap();
    let live = DomainCertificates::new(census).unwrap();
    assert_eq!(owned.occurrence_visits.get(), [72, 12]);
    compare_views(census, &owned, &live);
    assert_eq!(owned.occurrence_visits.get(), [72, 12]);
    assert_eq!(captures.get(), 4);
    assert!(live.occurrence_visits.get()[0] > 72);
    assert!(live.occurrence_visits.get()[1] > 12);
    check_actual_kinds(version, signature(&owned, census));
}
fn compare_views(
    census: &BoundaryCensus,
    owned: &DomainCertificates<'_>,
    live: &DomainCertificates<'_>,
) {
    for _ in 0..3 {
        assert_eq!(signature(owned, census), signature(live, census));
    }
}
fn check_actual_kinds(version: u32, signatures: Vec<String>) {
    if version == 6 {
        assert!(
            signatures
                .iter()
                .any(|value| value.starts_with("geometric:"))
        );
        return;
    }
    assert!(
        signatures
            .iter()
            .any(|value| value.starts_with("excluded:"))
    );
    assert!(signatures.iter().any(|value| value.starts_with("solver:")));
}
#[test]
fn conditioned_owned_epochs_expire_and_borrowed_views_recheck_native_curve_aliases() {
    for version in 3..=6 {
        check_native_alias(version);
    }
}
fn check_native_alias(version: u32) {
    let prepared = prepared(version);
    let census = &prepared.bridge.census;
    let captures = Cell::new(0);
    let owner = OwnedGraphAccess::fresh(&prepared.request, &prepared.edges, &captures);
    let live = DomainCertificates::new(census).unwrap();
    assert!(live.unified_certificate(&census.domains[0]).is_some());
    assert!(
        SourceAccess::Owned(&owner)
            .certificates(census)
            .unwrap()
            .unified_certificate(&census.domains[0])
            .is_some()
    );
    let before = captures.get();
    recapture::change_curve(census);
    let next = SourceAccess::Owned(&owner).certificates(census).unwrap();
    assert!(next.unified_certificate(&census.domains[0]).is_none());
    assert!(live.unified_certificate(&census.domains[0]).is_none());
    assert!(captures.get() > before);
}
