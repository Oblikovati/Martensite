// SPDX-License-Identifier: GPL-2.0-only
//! Forward #555 without changing genuine exclusion/fallback evidence or cancellation.
use super::*;
use crate::sketch_region_certification::DomainEvidence;
fn selected(version: u32) -> OwnedProfileRequest {
    let mut request = request();
    request.version = version;
    request
}
fn prepared(version: u32) -> super::super::super::PreparedProfile {
    super::super::super::pipeline::prepare_owned_profile(selected(version), &mut Continue)
        .unwrap_or_else(|_| panic!("actual unified owned fixture preparation"))
}
#[test]
fn real_first_exclusions_and_solver_fallbacks_share_only_one_actual_source_scan() {
    for version in [3, 4] {
        let prepared = prepared(version);
        let captures = std::cell::Cell::new(0);
        let access = OwnedGraphAccess::fresh(&prepared.request, &prepared.edges, &captures);
        let census = &prepared.bridge.census;
        let view = SourceAccess::Owned(&access).certificates(census).unwrap();
        check_unified_domains(census, &view);
        assert_eq!(captures.get(), census.inputs.segments.len());
    }
}
fn check_unified_domains(
    census: &BoundaryCensus,
    view: &crate::sketch_region_certification::DomainCertificates<'_>,
) {
    assert!(matches!(
        census.domains[0].evidence,
        DomainEvidence::BernsteinReport { .. }
    ));
    let first = view.unified_certificate(&census.domains[0]).unwrap();
    assert_eq!(first.contact_count(), 0);
    assert!(first.solver().is_none());
    assert!(
        census
            .domains
            .iter()
            .all(|domain| view.unified_certificate(domain).is_some())
    );
    assert!(census.domains.iter().any(|domain| {
        view.unified_certificate(domain)
            .and_then(|proof| proof.solver())
            .is_some()
    }));
    assert!(census.paired.as_ref().unwrap().reports.is_empty());
}
#[test]
fn selected_unified_owned_producer_keeps_live_bytes_trace_and_linear_captures() {
    for version in [3, 4] {
        let mut captures = 0;
        let mut actual_trace = Trace::default();
        let actual = run_observed(selected(version), &mut actual_trace, |count| {
            captures = count
        })
        .unwrap();
        let mut expected_trace = Trace::default();
        let expected = live(selected(version), &mut expected_trace).unwrap();
        assert_eq!(actual, expected);
        assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
        assert_eq!(actual_trace.checkpoints, expected_trace.checkpoints);
        assert_eq!(captures, 6 * selected(version).edges.len());
    }
}
#[test]
fn first_unified_binding_refusal_does_not_skip_ahead_to_a_later_owned_domain() {
    let mut prepared = prepared(3);
    prepared.bridge.census.domains[0].occurrences = [0, 1];
    let captures = std::cell::Cell::new(0);
    let access = OwnedGraphAccess::fresh(&prepared.request, &prepared.edges, &captures);
    let census = &prepared.bridge.census;
    let view = SourceAccess::Owned(&access).certificates(census).unwrap();
    assert!(view.unified_certificate(&census.domains[0]).is_none());
    assert!(view.unified_certificate(&census.domains[1]).is_some());
    assert_eq!(captures.get(), 0);
    compare_unified_refusal(&prepared, SourceAccess::Owned(&access));
}
fn compare_unified_refusal(
    prepared: &super::super::super::PreparedProfile,
    access: SourceAccess<'_>,
) {
    let actual = super::super::super::regions::inspect_graph(
        &prepared.request,
        &prepared.edges,
        &prepared.bridge,
        || false,
        access,
    )
    .unwrap_err();
    let expected = prepared.inspect_plane_graph(|| false).unwrap_err();
    assert_eq!(actual, expected);
    assert!(matches!(actual.cause, crate::sketch_region_prism::region_certificate::loop_region::LoopRegionCause::PairUnavailable { .. }));
}
#[test]
fn different_requested_selector_cannot_mint_owned_unified_source_authority() {
    let prepared = prepared(3);
    let mut changed = prepared.request.clone();
    changed.version = 2;
    let captures = std::cell::Cell::new(0);
    let access = OwnedGraphAccess::fresh(&changed, &prepared.edges, &captures);
    assert!(!access.binds(&prepared.bridge.census));
    let view = SourceAccess::Owned(&access)
        .certificates(&prepared.bridge.census)
        .unwrap();
    assert!(
        view.unified_certificate(&prepared.bridge.census.domains[0])
            .is_some()
    );
    assert_eq!(captures.get(), 0);
}
fn live_cancelled(
    request: OwnedProfileRequest,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> Result<GraphRegionCertificate, super::super::super::certify::ProfileProofRefusal> {
    let prepared = super::super::super::pipeline::prepare_owned_profile(
        request,
        &mut super::super::super::certify::PollProgress(cancelled),
    )
    .map_err(super::super::super::certify::preparation_refusal)?;
    prepared
        .inspect_plane_graph(cancelled)
        .map_err(|failure| super::super::super::certify::inspection_refusal(&prepared, failure))
}
#[test]
fn cancellation_in_final_actual_graph_checkpoint_preserves_live_cause_and_work() {
    let polls = completed_graph_polls();
    let mut actual = 0;
    let failure = run_with_cancel(selected(3), &mut || {
        actual += 1;
        actual == polls
    })
    .unwrap_err();
    let mut expected = 0;
    let live = live_cancelled(selected(3), &mut || {
        expected += 1;
        expected == polls
    })
    .unwrap_err();
    assert_eq!(failure, live);
    assert!(failure.cancelled);
    assert_eq!(actual, polls);
    assert_eq!(expected, polls);
}

fn completed_graph_polls() -> usize {
    let mut polls = 0;
    run_with_cancel(selected(3), &mut || {
        polls += 1;
        false
    })
    .unwrap();
    polls
}
