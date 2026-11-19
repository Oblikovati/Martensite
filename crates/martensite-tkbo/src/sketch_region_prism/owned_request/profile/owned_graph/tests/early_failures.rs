// SPDX-License-Identifier: GPL-2.0-only
//! #555: earlier live refusals stay earlier than owned validation or matrix diagnostics.
use super::*;
use crate::sketch_region::RegionSourceId;
use crate::sketch_region_certification::{
    CensusGap, CertificationObligation, DomainCertificates, DomainEvidence,
};
use crate::sketch_region_prism::region_certificate::loop_region::{
    LoopRegionCause, LoopRegionFailure, LoopRegionStage,
};
use martensite_tkernel::standard::Failure;
use std::cell::Cell;

fn prepared() -> super::super::super::PreparedProfile {
    super::super::super::pipeline::prepare_owned_profile(request(), &mut Continue)
        .unwrap_or_else(|_| panic!("fresh owned early-refusal fixture preparation"))
}
fn expected_sources(
    prepared: &super::super::super::PreparedProfile,
    domain: usize,
) -> [RegionSourceId; 2] {
    prepared.bridge.census.domains[domain]
        .occurrences
        .map(|index| RegionSourceId(prepared.request.edges[index].source.0))
}
fn same_refusal(
    prepared: &super::super::super::PreparedProfile,
    captures: &Cell<usize>,
) -> LoopRegionFailure {
    let expected = prepared.inspect_plane_graph(|| false).unwrap_err();
    let access = OwnedGraphAccess::fresh(&prepared.request, &prepared.edges, captures);
    let actual = super::super::super::regions::inspect_graph(
        &prepared.request,
        &prepared.edges,
        &prepared.bridge,
        || false,
        SourceAccess::Owned(&access),
    )
    .unwrap_err();
    assert_eq!(actual, expected);
    actual
}
fn first_pair(failure: LoopRegionFailure, sources: [RegionSourceId; 2]) {
    assert_eq!(failure.stage, LoopRegionStage::Joins);
    assert_eq!(failure.cause, LoopRegionCause::PairUnavailable { sources });
}
#[test]
fn unsupported_or_missing_first_report_preserves_live_pair_context_without_owned_capture() {
    for missing in [false, true] {
        let mut prepared = prepared();
        make_first_unavailable(&mut prepared.bridge.census, missing);
        let sources = expected_sources(&prepared, 0);
        let captures = Cell::new(0);
        first_pair(same_refusal(&prepared, &captures), sources);
        assert_eq!(captures.get(), 0);
    }
}
fn make_first_unavailable(census: &mut BoundaryCensus, missing: bool) {
    if !missing {
        census.domains[0].evidence = DomainEvidence::Unsupported(CensusGap::BezierPair);
        return;
    }
    let DomainEvidence::PairedReport { report, .. } = &mut census.domains[0].evidence else {
        panic!("fresh first domain must retain an actual paired report");
    };
    *report = usize::MAX;
}
#[test]
fn valid_first_report_then_invalid_domain_precedes_aggregate_matrix_obligations() {
    let mut prepared = prepared();
    prepared.bridge.census.domains[1].occurrences = [0, 2];
    let census = &prepared.bridge.census;
    let view = DomainCertificates::new(census).unwrap();
    assert!(view.certificate(&census.domains[0]).is_some());
    assert!(view.certificate(&census.domains[1]).is_none());
    assert!(
        census
            .current_obligations()
            .contains(&CertificationObligation::ExhaustivePairAndSelfDomains)
    );
    let captures = Cell::new(0);
    first_pair(
        same_refusal(&prepared, &captures),
        expected_sources(&prepared, 1),
    );
    assert_eq!(captures.get(), census.inputs.segments.len());
}
#[test]
fn corrupt_work_or_source_policy_cannot_admit_owned_cache_or_reorder_live_refusals() {
    for policy in [false, true] {
        let mut prepared = prepared();
        corrupt_before_view(&mut prepared.bridge.census, policy);
        let captures = Cell::new(0);
        refuse_owned_cache(&prepared, &captures);
        first_pair(
            same_refusal(&prepared, &captures),
            expected_sources(&prepared, 0),
        );
        assert_eq!(captures.get(), 0);
    }
}
fn corrupt_before_view(census: &mut BoundaryCensus, policy: bool) {
    if policy {
        census.source_policy.as_mut().unwrap().construction_limit = 1e-4;
        return;
    }
    census.bisections += 1;
}
fn refuse_owned_cache(prepared: &super::super::super::PreparedProfile, captures: &Cell<usize>) {
    let access = OwnedGraphAccess::fresh(&prepared.request, &prepared.edges, captures);
    assert!(
        SourceAccess::Owned(&access)
            .certificates(&prepared.bridge.census)
            .is_none()
    );
    assert_eq!(captures.get(), 0);
}
#[derive(Default)]
struct CallbackFailure {
    checkpoints: Vec<OwnedPrismCheckpoint>,
    target: usize,
    failed: Option<OwnedPrismCheckpoint>,
}
impl OwnedPrismProgress for CallbackFailure {
    fn checkpoint(&mut self, point: OwnedPrismCheckpoint) -> Result<bool, Failure> {
        self.checkpoints.push(point);
        if point.ordinal == self.target {
            self.failed = Some(point);
            return Err(Failure::new(
                "owned graph callback value=failed; expected continuation",
            ));
        }
        Ok(false)
    }
}
fn in_stage_checkpoint() -> OwnedPrismCheckpoint {
    let mut trace = Trace::default();
    run(request(), &mut trace).unwrap();
    trace
        .checkpoints
        .windows(2)
        .find_map(|pair| {
            let census = OwnedPrismPhase::Census;
            (pair[0].phase == census && pair[1].phase == census).then_some(pair[1])
        })
        .expect("actual census-internal checkpoint after phase entry")
}
#[test]
fn callback_native_failure_inside_census_matches_live_failed_class_phase_and_trace() {
    let point = in_stage_checkpoint();
    let mut actual = callback_at(point);
    let mut expected = callback_at(point);
    let actual_failure = run(request(), &mut actual).unwrap_err();
    let expected_failure = live(request(), &mut expected).unwrap_err();
    assert_eq!(actual_failure, expected_failure);
    assert_eq!(actual.checkpoints, expected.checkpoints);
    assert_eq!(actual.failed, Some(point));
    assert_eq!(expected.failed, Some(point));
    let expected = format!(
        "profile preparation Failed at {:?}: stage refusal",
        point.phase
    );
    assert_eq!(
        actual_failure,
        super::super::super::certify::ProfileProofRefusal::Incomplete(expected)
    );
}
fn callback_at(point: OwnedPrismCheckpoint) -> CallbackFailure {
    CallbackFailure {
        target: point.ordinal,
        ..Default::default()
    }
}
